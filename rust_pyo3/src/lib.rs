use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::Python;

mod kmeans;
mod linfa_kmeans;
mod stratif_split;

#[pyfunction]
fn dist(x1: PyReadonlyArray1<f64>, x2: PyReadonlyArray1<f64>) -> f64 {
    let x1 = x1.as_array();
    let x2 = x2.as_array();
    let mut s: f64 = 0.0;
    for i in 0..x1.len() {
        let cur = (x1[i] - x2[i]).powi(2);
        s += cur
    }
    s.sqrt()
}

#[pymodule]
fn rust_pyo3(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dist,m)?)?;
    kmeans::register(m)?;
    linfa_kmeans::register(m)?;
    stratif_split::register(m)?;
    Ok(())
}
