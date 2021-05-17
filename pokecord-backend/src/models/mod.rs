//! Module containing various structs/Python classes to transport data between
//! Rust and Python code. The structs here should have minimal function
//! implementations and are intended to represent data only.
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Inits the model's module.
pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<Pokemon>()?;
    Ok(())
}

/// Class representing a single Pokemon, including relevant data fields.
#[pyclass(module = "pokecord_backend.models")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pokedex_id: u16,
    species: String,
    nickname: Option<String>,
    description: String,
    primary_type: PokemonType,
    secondary_type: Option<PokemonType>,
    current_xp: u32,
    level: u8,
    next_level_xp: u32,
    evolution_level: Option<u8>,
    capture_timestamp: u64,
    image_path: String,
}

/// An enum of all Pokemon types.
/// This will be converted to lower case strings of the type when sent to Python.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum PokemonType {
    NORMAL,
    FIRE,
    WATER,
    ELECTRIC,
    GRASS,
    ICE,
    FIGHTING,
    POISON,
    GROUND,
    FLYING,
    PSYCHIC,
    BUG,
    ROCK,
    GHOST,
    DRAGON,
    DARK,
    STEEL,
    FAIRY,
}

impl IntoPy<PyObject> for PokemonType {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            PokemonType::NORMAL => "normal".into_py(py),
            PokemonType::FIRE => "fire".into_py(py),
            PokemonType::WATER => "water".into_py(py),
            PokemonType::ELECTRIC => "electric".into_py(py),
            PokemonType::GRASS => "grass".into_py(py),
            PokemonType::ICE => "ice".into_py(py),
            PokemonType::FIGHTING => "fighting".into_py(py),
            PokemonType::POISON => "poison".into_py(py),
            PokemonType::GROUND => "ground".into_py(py),
            PokemonType::FLYING => "flying".into_py(py),
            PokemonType::PSYCHIC => "psychic".into_py(py),
            PokemonType::BUG => "bug".into_py(py),
            PokemonType::ROCK => "rock".into_py(py),
            PokemonType::GHOST => "ghost".into_py(py),
            PokemonType::DRAGON => "dragon".into_py(py),
            PokemonType::DARK => "dark".into_py(py),
            PokemonType::STEEL => "steel".into_py(py),
            PokemonType::FAIRY => "fairy".into_py(py),
        }
    }
}
