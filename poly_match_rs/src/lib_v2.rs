use pyo3::prelude::*;

use ndarray::Array1;
use ndarray_linalg::Norm;
use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};

#[pyclass(subclass)]
struct Polygon {
    // x: Array1<f64>,
    // // TODO (define y and center)
}

#[pymethods]
impl Polygon {
    // #[new]
    // fn new(x: PyReadonlyArray1<f64>, y: PyReadonlyArray1<f64>) -> Polygon {
    //     let x = // TODO (convert x to array)
    //     let y = // TODO (convert y to array)
    //     let center = // TODO (retrieve the mean of x and y into an array)

    //     Polygon {
    //         x: x.to_owned(),
    //         y: y.to_owned(),
    //         center,
    //     }
    // }

    // #[getter]
    // fn x<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
    //     Ok(self.x.to_pyarray_bound(py))
    // }

    // // TODO (getter for y and center)
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
        // let center = // TODO (borrow center from poly)

        // if (center - point).norm() < max_dist {
        //     close_polygons.push(poly)
        // }
    }

    Ok(close_polygons)
}

pub fn poly_match_rs(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Polygon>()?;
    m.add_function(wrap_pyfunction!(find_close_polygons, m)?)?;
    Ok(())
}
