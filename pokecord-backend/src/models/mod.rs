//! Module containing various structs/Python classes to transport data between
//! Rust and Python code. The structs here should have minimal function
//! implementations and are intended to represent data only.
use std::convert::{TryFrom, TryInto};

use crate::{Pokemon as ApiPokemon, PokemonSpecies};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

const DEFAULT_LEVEL: u8 = 5;

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

impl Pokemon {
    pub fn from_api_resources(api_pokemon: ApiPokemon, species: PokemonSpecies) -> Pokemon {
        let secondary_type: Option<PokemonType>;
        if api_pokemon.types.len() > 1 {
            let secondary_type_str = &api_pokemon.types[1].damage_type.name;
            secondary_type = Some(secondary_type_str.try_into().unwrap());
        } else {
            secondary_type = None;
        }

        Pokemon {
            pokedex_id: api_pokemon.id as u16,
            species: api_pokemon.name,
            nickname: None,
            description: species.flavor_text_entries[1].flavor_text.clone(),
            primary_type: (&api_pokemon.types[0].damage_type.name).try_into().unwrap(),
            secondary_type,
            level: DEFAULT_LEVEL,
            current_xp: api_pokemon.base_experience as u32,
            next_level_xp: 100000,
            evolution_level: None,
            capture_timestamp: 0,
            image_path: String::from(""),
        }
    }
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

impl TryFrom<&String> for PokemonType {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let pokemon_type = match value.as_str() {
            "normal" => Self::NORMAL,
            "fire" => Self::FIRE,
            "water" => Self::WATER,
            "electric" => Self::ELECTRIC,
            "grass" => Self::GRASS,
            "ice" => Self::ICE,
            "fighting" => Self::FIGHTING,
            "poison" => Self::POISON,
            "ground" => Self::GROUND,
            "flying" => Self::FLYING,
            "psychic" => Self::PSYCHIC,
            "bug" => Self::BUG,
            "rock" => Self::ROCK,
            "ghost" => Self::GHOST,
            "dragon" => Self::DRAGON,
            "dark" => Self::DARK,
            "steel" => Self::STEEL,
            "fairy" => Self::FAIRY,
            _ => {
                return Err(format!(
                    "Conversion failed from {}. Unknown pokemon type.",
                    value
                ));
            }
        };
        Ok(pokemon_type)
    }
}
