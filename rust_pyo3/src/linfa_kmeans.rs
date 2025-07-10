// use linfa::prelude::*;
// use linfa_clustering::KMeans;
// use linfa_nn::distance::L2Dist;
// use ndarray::prelude::*;
// use numpy::PyReadonlyArray2;
// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
// use rand::prelude::*;

// #[pyfunction]
// fn linfa_kmeans_test(n_clusters_: usize, data: PyReadonlyArray2<f64>) -> PyResult<Vec<Vec<f64>>> {
//     let data = data.as_array();

//     let targets = Array::from_elem(data.nrows(), ());
//     let dataset = DatasetBase::new(data, targets);

//     let rng = rand::rng(); // Random number generator
//     let n_clusters = n_clusters_;
//     let model = KMeans::params_with(n_clusters, rng, L2Dist)
//         .max_n_iterations(300)
//         .tolerance(1e-6)
//         .fit(&dataset)
//         .expect("Error while fitting KMeans to the dataset");

//     // let dataset = model.predict(dataset);

//     let centroids = model.centroids();
//     let cluster_centers_: Vec<Vec<f64>> = centroids.outer_iter().map(|row| row.to_vec()).collect();

//     Ok(cluster_centers_)
// }

// pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(euclidean, m)?)?;
//     m.add_function(wrap_pyfunction!(linfa_kmeans_test, m)?)?;
//     Ok(())
// }
