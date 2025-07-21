use linfa::prelude::*;
use linfa_clustering::KMeans;
use ndarray::prelude::*;
use numpy::{PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn linfa_kmeans_test<'py>(
    py: Python<'py>,
    data: PyReadonlyArray2<'py, f64>,
    n_clusters: i32,
    max_iter: i32,
    tol: f64,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let n_clusters_usize = n_clusters as usize;

    let data = data.as_array();
    let targets = Array::from_elem(data.nrows(), ());
    let dataset = DatasetBase::new(data, targets);

    let model = KMeans::params(n_clusters_usize)
        .max_n_iterations(max_iter as u64)
        .tolerance(tol)
        .fit(&dataset)
        .expect("KMeans fitted");

    // let dataset = model.predict(dataset);

    let centroids = model.centroids();

    let centroids_py = PyArray2::from_owned_array(py, centroids.to_owned());
    Ok(centroids_py.into())
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(linfa_kmeans_test, m)?)?;
    Ok(())
}
