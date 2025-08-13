use pyo3::prelude::*;

use ndarray_linalg::Norm;
use numpy::PyReadonlyArray1;

#[pyfunction]
fn find_close_polygons<'py>(
    polygons: Vec<Bound<'py, PyAny>>,
    point: PyReadonlyArray1<'py, f64>,
    max_dist: f64,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    // let mut close_polygons = // TODO (create an empty list)
    // let point = // TODO ()
    // for poly in polygons {
    //     let center = poly
    //         .getattr("center")?
    //         .extract::<PyReadonlyArray1<f64>>()?
    //         .as_array()
    //         .to_owned();

    //     if // TODO (distance between center and point is smaller than max_dist) {
    //         // TODO (push poly to close_polygons)
    //     }
    // }

    Ok(vec![]) // TODO
}

pub fn poly_match_rs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // TODO
    Ok(())
}
