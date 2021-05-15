use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};

use pokedex::{Pokedex, Pokemon};
use tracing::Level;

use crate::pokedex::PokemonSpecies;

mod pokedex;

create_exception!(
    pokecord_backend,
    PokedexError,
    pyo3::exceptions::PyException
);

/// add(a, b, /)
/// --
///
/// This function adds two unsigned 64-bit integers.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
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

/// A Python module implemented in Rust.
#[pymodule]
fn pokecord_backend(py: Python, m: &PyModule) -> PyResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(true)
        // .pretty()
        .init();
    tracing::info!("PokeCord Backend {}", env!("CARGO_PKG_VERSION"));

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(list_pokemon, m)?)?;
    m.add("PokedexError", py.get_type::<PokedexError>())?;

    Ok(())
}
