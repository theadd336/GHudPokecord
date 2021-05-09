use crate::models::Pokemon;
use pyo3::prelude::*;

mod handlers;

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_function(pyo3::wrap_pyfunction!(register_player, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(get_starter_pokemon_list, module)?)?;
    module.add_function(pyo3::wrap_pyfunction!(is_player_registered, module)?)?;
    Ok(())
}

/// add(a, b, /)
/// --
///
/// This function adds two unsigned 64-bit integers.
#[pyfunction]
fn register_player(player_id: &str, starter_pokemon: &Pokemon) -> PyResult<()> {
    Ok(())
}

#[pyfunction]
fn get_starter_pokemon_list() -> PyResult<Vec<Pokemon>> {
    Ok(vec![])
}

#[pyfunction]
fn is_player_registered(player_id: &str) -> PyResult<bool> {
    Ok(false)
}
