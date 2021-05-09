use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod database;
mod models;
mod registration;

/// A Python module implemented in Rust.
#[pymodule]
fn pokecord_backend(py: Python, m: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "registration")?;
    registration::init_submodule(submod)?;
    m.add_submodule(submod)?;

    let submod = PyModule::new(py, "models")?;
    models::init_submodule(submod)?;
    m.add_submodule(submod)?;
    Ok(())
}
