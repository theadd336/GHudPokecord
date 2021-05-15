use std::time::Duration;

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

#[pyfunction]
fn list_pokemon() -> PyResult<Vec<String>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let mut client = Pokedex::new();

        println!("Species (raw name):");
        let all_species = client.list::<PokemonSpecies>().await?;
        for species in all_species.iter() {
            println!("- {}", species.name);
        }

        let pokemon = client
            .get_by_name::<Pokemon>("mudkip")
            .await
            .expect("No mudkip?");
        dbg!(&pokemon);

        let species = client
            .get_by_ref(&pokemon.species)
            .await
            .expect("No species");
        println!("Species name: {}", species.name);
        for name in species.names.iter().filter(|n| n.language.name == "en") {
            println!("- {}", name.name);
        }

        println!("Flavor text:");
        for flavor_text in species
            .flavor_text_entries
            .iter()
            .filter(|f| f.language.name == "en")
        {
            println!("- {}", flavor_text.flavor_text);
        }

        Ok(all_species.into_iter().map(|s| s.name).collect())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pokecord_backend(py: Python, m: &PyModule) -> PyResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_ansi(true)
        .pretty()
        .init();
    tracing::info!("PokeCord Backend {}", env!("CARGO_PKG_VERSION"));

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(list_pokemon, m)?)?;
    m.add("PokedexError", py.get_type::<PokedexError>())?;

    Ok(())
}
