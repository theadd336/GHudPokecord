//! The pokecord backend library. All business logic and database access for
//! the pokecord discord frontend exists here.

use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3_asyncio::tokio as pytokio;
use pyo3_log;
use tracing::Level;

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

// This is an example of using the Pokemon API. It creates its own Tokio runtime and is generally pretty janky.
#[pyfunction]
fn list_pokemon() -> PyResult<Vec<String>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut client = Pokedex::new();

        let all_species = client.list::<PokemonSpecies>().await?;

        let pokemon = client
            // .get_by_name::<Pokemon>("mudkip")
            .get_by_id::<Pokemon>(12)
            .await
            .expect("Could not get Pokemon!");
        tracing::info!("{:?}", pokemon);
        tracing::info!("Image URL: {}", pokedex::image_url(pokemon.id));

        let species = client
            .get_by_ref(&pokemon.species)
            .await
            .expect("No species");

        let name = species.names.iter().find_map(|n| {
            if n.language.name == "en" {
                Some(n.name.as_str())
            } else {
                None
            }
        });
        tracing::info!("Species name: {}", name.unwrap_or("<unknown>"));

        Ok(all_species.into_iter().map(|s| s.name).collect())
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
