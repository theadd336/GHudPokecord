//! The `registration` module contains code associated with adding a palyer to
//! the game for the first time, as well as validating if a player has already
//! joined. This module also contains functions for getting the list of starter
//! pokemon.

use crate::database::{self, DatabaseError};
use crate::models::Pokemon;
use crate::pokedex::{Pokedex, Pokemon as ApiPokemon, PokemonSpecies, STARTER_POKEMON};
use pyo3::{
    exceptions::{PyKeyError, PyRuntimeError},
    prelude::*,
};
use pyo3_asyncio::tokio as pytokio;

// Adds all required functions into the module.
// TODO: Macro for this to remove boilerplate?
pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(register_player, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(get_starter_pokemon_list, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(is_player_registered, module)?)?;
    Ok(())
}

/// Registers a player into the game for the first time.
/// This function will return None on success and raise a
/// `KeyError` on failure.
#[pyfunction]
#[text_signature = "(player_id, starter, /)"]
fn register_player(py: Python, player_id: String, starter: Pokemon) -> PyResult<PyObject> {
    log::debug!(
        "Entering PyO3 bridge to register_player with args ({}, {:?})",
        player_id,
        starter
    );
    pytokio::into_coroutine(py, async move {
        if let Err(e) = database::register_player(&player_id, &starter).await {
            log::error!("An error occurred while registering player {}", player_id);
            match e {
                DatabaseError::PlayerAlreadyExists => {
                    return Python::with_gil(|_| {
                        Err(PyKeyError::new_err(format!(
                            "Player {} is already registered",
                            player_id
                        )))
                    })
                }
                _ => {
                    return Python::with_gil(|_| {
                        Err(PyRuntimeError::new_err(
                            "An internal error occurred. Check logs for details.",
                        ))
                    })
                }
            }
        }
        Python::with_gil(|py| Ok(py.None()))
    })
}

/// Fetches the starter pokemon list.
///
/// # Returns
///
/// A `list` of `Pokemon`
#[pyfunction]
#[text_signature = "(/)"]
fn get_starter_pokemon_list(py: Python) -> PyResult<PyObject> {
    log::debug!("Entering PyO3 bridge to get starter pokemon");
    pytokio::into_coroutine(py, async move {
        let mut starter_pokemon = Vec::with_capacity(STARTER_POKEMON.len());
        let mut pokedex = Pokedex::new();
        for &pokemon_species in STARTER_POKEMON {
            let api_pokemon: ApiPokemon = pokedex.get_by_name(pokemon_species).await.unwrap();
            let pokemon_species: PokemonSpecies =
                pokedex.get_by_name(pokemon_species).await.unwrap();
            let pokemon = Pokemon::from_api_resources(api_pokemon, pokemon_species);
            starter_pokemon.push(pokemon);
        }
        Python::with_gil(|py| Ok(starter_pokemon.into_py(py)))
    })
}

/// Checks if a given player is registered for the game already.
///
/// # Returns
///
/// `True` if the player is registered. `False` otherwise.
#[pyfunction]
#[text_signature = "(player_id, /)"]
fn is_player_registered(py: Python, player_id: &str) -> PyResult<bool> {
    Ok(false)
}
