use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

fn dist(x1: ArrayView1<f64>, x2: ArrayView1<f64>) -> f64 {
    // let x1 = x1.as_array();
    // let x2 = x2.as_array();
    let mut s: f64 = 0.0;
    for i in 0..x1.len() {
        let cur = (x1[i] - x2[i]).powi(2);
        s += cur
    }
    s.sqrt()
}

fn init_kmeans_pp(data: ArrayView2<f64>, n_clusters: i32) -> Array2<f64> {
    let n_samples = data.shape()[0];
    let n_features = data.shape()[1];
    let n_clusters_usize = n_clusters as usize;

    // Chooses first centroid randomly
    let mut centroids = Array2::<f64>::zeros((n_clusters_usize, n_features));
    let mut rng = rand::rng();
    centroids
        .row_mut(0)
        .assign(&data.row(rng.random_range(0..n_samples)));

    for k in 0..n_clusters_usize {
        // Computes distances to the nearest centroid
        let mut distances = Vec::new();
        for x_i in data.rows() {
            let mut min_dist = f64::INFINITY;
            for j in 0..k {
                let dist = dist(x_i, centroids.row(j));
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            distances.push(min_dist);
        }

        // Defines proportionnal probabilities according to distances
        let total_dist: f64 = distances.iter().sum();
        let mut probabilities = Vec::new();
        for &distance in &distances {
            probabilities.push(distance / total_dist);
        }

        // Computes a new centroid
        let mut cumulative_prob = Vec::new();
        let mut sum = 0.0;
        for &p in &probabilities {
            sum += p;
            cumulative_prob.push(sum);
        }
        let r: f64 = rng.random();
        for (i, &cum_prob) in cumulative_prob.iter().enumerate() {
            if r < cum_prob {
                centroids.row_mut(k).assign(&data.row(i));
                break;
            }
        }
    }
    centroids
}

#[pyfunction]
fn kmeans_alamano<'py>(
    py: Python<'py>,
    data: PyReadonlyArray2<'py, f64>,
    n_clusters: i32,
    max_iter: Option<i32>,
    tol: Option<f64>,
    init: Option<&str>,
    debug: Option<bool>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    // Default parameters
    let max_iter = max_iter.unwrap_or(300);
    let tol = tol.unwrap_or(1e-6);
    let init = init.unwrap_or("None");
    let debug = debug.unwrap_or(true);
    let n_clusters_usize = n_clusters as usize;

    // Constants of the dataset
    let data = data.as_array();
    let n_samples = data.shape()[0];
    let n_features = data.shape()[1];

    // Initializes centroids
    let mut centroids = Array2::<f64>::zeros((n_clusters_usize, n_features));
    let mut prev_centroids = Array2::<f64>::zeros((n_clusters_usize, n_features));
    let mut rng = rand::rng();
    for i in 0..n_clusters_usize {
        let random_index = rng.random_range(0..n_samples);
        centroids.row_mut(i).assign(&data.row(random_index));
    }
    if init == "kmeans++" {
        centroids = init_kmeans_pp(data, n_clusters);
    }

    // Iterations of Kmeans
    for iteration in 0..max_iter {
        // Computes clusters according to the centroids
        let mut clusters = Vec::new();
        for _ in 0..n_clusters {
            clusters.push(Vec::new());
        }
        for x_i in data.rows() {
            let mut to_k: usize = 0;
            let mut dist_min = f64::INFINITY;
            for k in 0..n_clusters_usize {
                let dist_to_k = dist(x_i, centroids.row(k));
                if dist_to_k < dist_min {
                    to_k = k;
                    dist_min = dist_to_k;
                }
            }
            clusters[to_k].push(x_i);
        }

        // Computes centroids
        for k in 0..n_clusters_usize {
            let mut centroids_k = Array1::<f64>::zeros(n_features);
            for x_ik in &clusters[k] {
                centroids_k += x_ik;
            }
            let cluster_size = clusters[k].len();
            let divisor = if cluster_size > 1 {
                cluster_size as f64
            } else {
                1.0
            };
            centroids_k /= divisor;
            centroids.row_mut(k).assign(&centroids_k);
        }

        // Checks the convergence
        let mut dist_centroids = 0.0;
        for k in 0..n_clusters_usize {
            dist_centroids += dist(centroids.row(k), prev_centroids.row(k));
        }
        if dist_centroids < tol {
            if debug {
                println!("{}", iteration);
            }
            break;
        }

        // Keeps previous centroids in memory
        prev_centroids.assign(&centroids);
    }

    let centroids_py = PyArray2::from_owned_array(py, centroids);
    Ok(centroids_py.into())
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kmeans_alamano, m)?)?;
    Ok(())
}
