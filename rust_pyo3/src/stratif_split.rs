use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use numpy::{PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::Rng;

#[pyfunction]
fn stratif_train_test_split(data: PyReadonlyArray2<f64>, labels: PyReadonlyArray1<i32>, test_size: f64, train_size: f64) {
    
    return;
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stratif_train_test_split, m)?)?;
    Ok(())
}
