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
    #[new]
    fn new(x: PyReadonlyArray1<f64>, y: PyReadonlyArray1<f64>) -> Polygon {
        // TODO
    }

    #[getter]
    fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.x.to_pyarray_bound(py))
    }

    #[getter]
    fn y<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.y.to_pyarray_bound(py))
    }

    #[getter]
    fn center<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.center.to_pyarray_bound(py))
    }
}

#[pyfunction]
fn find_close_polygons<'py>(
    polygons: Vec<Bound<'py, Polygon>>,
    point: PyReadonlyArray1<'py, f64>,
    max_dist: f64,
) -> PyResult<Vec<Bound<'py, Polygon>>> {
    // TODO
}

pub fn poly_match_rs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // TODO
    Ok(())
}
