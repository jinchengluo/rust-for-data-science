use pyo3::prelude::*;

use ndarray_linalg::Norm;
use numpy::PyReadonlyArray1;

#[pyfunction]
fn find_close_polygons<'py>(
    polygons: Vec<Bound<'py, PyAny>>,
    point: PyReadonlyArray1<'py, f64>,
    max_dist: f64,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    // TODO
    Ok(vec![])
}

pub fn poly_match_rs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // TODO
    Ok(())
}
