//! The `registration` module contains code associated with adding a palyer to
//! the game for the first time, as well as validating if a player has already
//! joined. This module also contains functions for getting the list of starter
//! pokemon.

use crate::models::Pokemon;
use pyo3::prelude::*;
use pyo3_asyncio::tokio as pytokio;

mod handlers;

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
#[text_signature = "(player_id, /)"]
fn register_player(py: Python, player_id: &str) -> PyResult<PyObject> {
    pytokio::into_coroutine(py, async move { Python::with_gil(|py| Ok(py.None())) })
}

/// Fetches the starter pokemon list.
///
/// # Returns
///
/// A `list` of `Pokemon`
#[pyfunction]
#[text_signature = "(/)"]
fn get_starter_pokemon_list(py: Python) -> PyResult<Vec<Pokemon>> {
    log::error!("Here");
    Ok(vec![Pokemon {}])
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
