//! Module containing various structs/Python classes to transport data between
//! Rust and Python code. The structs here should have minimal function
//! implementations and are intended to represent data only.
use pyo3::prelude::*;

/// Inits the model's module.
pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<Pokemon>()?;
    Ok(())
}

/// Class representing a single Pokemon, including relevant data fields.
#[pyclass(module = "pokecord_backend.models")]
pub struct Pokemon {}
