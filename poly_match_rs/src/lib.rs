mod lib_v0;
mod lib_v1;
mod lib_v2;
mod lib_v3;
mod lib_v4;

use pyo3::prelude::*;

#[pymodule]
pub fn poly_match_rs(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    let v0 = PyModule::new_bound(py, "v0")?;
    lib_v0::poly_match_rs(py, &v0)?;
    m.add_submodule(&v0)?;

    let v1 = PyModule::new_bound(py, "v1")?;
    lib_v1::poly_match_rs(py, &v1)?;
    m.add_submodule(&v1)?;

    let v2 = PyModule::new_bound(py, "v2")?;
    lib_v2::poly_match_rs(py, &v2)?;
    m.add_submodule(&v2)?;

    let v3 = PyModule::new_bound(py, "v3")?;
    lib_v3::poly_match_rs(py, &v3)?;
    m.add_submodule(&v3)?;

    let v4 = PyModule::new_bound(py, "v4")?;
    lib_v4::poly_match_rs(py, &v4)?;
    m.add_submodule(&v4)?;

    Ok(())
}
