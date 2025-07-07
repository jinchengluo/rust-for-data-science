use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;
use ndarray::prelude::*;
use numpy::PyReadonlyArray2;
use rand::prelude::*;
// use plotters::prelude::*;
use pyo3::prelude::*;
use pyo3::Python;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn euclidean(x: Vec<f64>, y: Vec<f64>) -> PyResult<f64> {
    // if x.len() != y.len() {
    //     return Err(pyo3::exceptions::PyValueError::new_err("Vectors are not the same size."));
    // }
    let mut s : f64 = 0.0;
    for i in 1..x.len() {
        let cur = (x[i] - y[i]).powi(2);
        s += cur
    }
    Ok(s.sqrt())
}

#[pyfunction]
fn linfa_kmeans_test(n_clusters_ : usize, data: PyReadonlyArray2<f64>) -> PyResult<Vec<Vec<f64>>> {

    // fn create_square(center: [f64; 2], spread: f64, n: usize) -> Array2<f64> {
    //     let mut rng = rand::thread_rng();
    //     let mut data = Array2::<f64>::zeros((n, 2));
    //     for mut row in data.outer_iter_mut() {
    //         row[0] = center[0] + rng.gen_range(-spread..spread);
    //         row[1] = center[1] + rng.gen_range(-spread..spread);
    //     }
    //     return data
    // }

    // let square_1: Array2<f64> = create_square([7.0, 5.0], 1.0, 150); // Cluster 1
    // let square_2: Array2<f64> = create_square([2.0, 2.0], 2.0, 150); // Cluster 2
    // let square_3: Array2<f64> = create_square([3.0, 8.0], 1.0, 150); // Cluster 3
    // let square_4: Array2<f64> = create_square([5.0, 5.0], 9.0, 300); // A bunch of noise across them all

    // let data: Array2<f64> = ndarray::concatenate(
    //     Axis(0),
    //     &[
    //         square_1.view(),
    //         square_2.view(),
    //         square_3.view(),
    //         square_4.view(),
    //     ],
    // )
    // .expect("An error occurred while stacking the dataset");

    let data = data.as_array();

    let targets = Array::from_elem(data.nrows(), ());
    let dataset = DatasetBase::new(data, targets);

    let rng = thread_rng(); // Random number generator
    let n_clusters = n_clusters_;
    let model = KMeans::params_with(n_clusters, rng, L2Dist)
        .max_n_iterations(200)
        .tolerance(1e-5)
        .fit(&dataset)
        .expect("Error while fitting KMeans to the dataset");

    // let dataset = model.predict(dataset);

    let centroids = model.centroids();
    let cluster_centers_: Vec<Vec<f64>> = centroids
        .outer_iter()
        .map(|row| row.to_vec())
        .collect();

    Ok(cluster_centers_)
}

#[pymodule]
fn rust_pyo3(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(euclidean, m)?)?;
    m.add_function(wrap_pyfunction!(linfa_kmeans_test, m)?)?;
    Ok(())
}
