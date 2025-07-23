use pyo3::prelude::*;

use ndarray::Array1;
use ndarray_linalg::Norm;
use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};

#[pyclass(subclass)]
struct Polygon {
    // TODO
}

#[pymethods]
impl Polygon {
    // TODO (constructor)
    // TODO (getter for x, y and center)
}

#[pyfunction]
fn find_close_polygons<'py>(
    polygons: Vec<Bound<'py, Polygon>>,
    point: PyReadonlyArray1<'py, f64>,
    max_dist: f64,
) -> PyResult<Vec<Bound<'py, Polygon>>> {
    let mut close_polygons = vec![];
    let point = point.as_array();
    for poly in polygons {
        // TODO
    }

    Ok(close_polygons)
}

pub fn poly_match_rs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // TODO
    m.add_function(wrap_pyfunction!(find_close_polygons, m)?)?;
    Ok(())
}
