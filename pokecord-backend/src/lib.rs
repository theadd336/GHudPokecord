//! The pokecord backend library. All business logic and database access for
//! the pokecord discord frontend exists here.

use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3_asyncio::tokio as pytokio;
use pyo3_log;

use crate::pokedex::{Pokedex, Pokemon, PokemonSpecies};

mod database;
mod models;
mod pokedex;
mod registration;

create_exception!(
    pokecord_backend,
    PokedexError,
    pyo3::exceptions::PyException
);



/// Test function that tests logging at different levels to confirm config.
#[pyfunction]
fn test_logging() {
    log::debug!("This is a debug message.");
    log::info!("This is an info message");
    log::warn!("This is a warning message");
    log::error!("This is an error message");
}

// This lists out all pokemon by name
#[pyfunction]
fn list_pokemon(py: Python) -> PyResult<PyObject> {
    pytokio::into_coroutine(py, async {
        let mut client = Pokedex::new();
        let all_pokemon = client.list::<Pokemon>().await?;
        let names: Vec<_> = all_pokemon.into_iter().map(|s| s.name).collect();
        Ok(Python::with_gil(|py| names.into_py(py)))
    })
}
    
/// Main entry point for all python code. This function represents the
/// root Python module. All submodules should be added here.
/// TODO: Another macro maybe?
#[pymodule]
fn pokecord_backend(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    pytokio::init_multi_thread();
    pyo3_asyncio::try_init(py)?;
    m.add_function(wrap_pyfunction!(test_logging, m)?)?;
    m.add_function(wrap_pyfunction!(list_pokemon, m)?)?;
    m.add("PokedexError", py.get_type::<PokedexError>())?;
    let submod = PyModule::new(py, "registration")?;
    registration::init_submodule(submod)?;
    m.add_submodule(submod)?;

    let submod = PyModule::new(py, "models")?;
    models::init_submodule(submod)?;
    m.add_submodule(submod)?;
    Ok(())
}
