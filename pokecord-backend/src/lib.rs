//! The pokecord backend library. All business logic and database access for
//! the pokecord discord frontend exists here.

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_asyncio::tokio;
use pyo3_log;

mod database;
mod models;
mod registration;

/// Test function that tests logging at different levels to confirm config.
#[pyfunction]
fn test_logging() {
    log::debug!("This is a debug message.");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
}

/// Main entry point for all python code. This function represents the
/// root Python module. All submodules should be added here.
/// TODO: Another macro maybe?
#[pymodule]
fn pokecord_backend(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    tokio::init_multi_thread();
    pyo3_asyncio::try_init(py)?;
    m.add_function(wrap_pyfunction!(test_logging, m)?)?;
    let submod = PyModule::new(py, "registration")?;
    registration::init_submodule(submod)?;
    m.add_submodule(submod)?;

    let submod = PyModule::new(py, "models")?;
    models::init_submodule(submod)?;
    m.add_submodule(submod)?;
    Ok(())
}
