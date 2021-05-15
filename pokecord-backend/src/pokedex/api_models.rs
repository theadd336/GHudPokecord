//! Types for deserializing PokeAPI responses.

use std::{fmt::Debug, marker::PhantomData};

use serde::{Deserialize, Serialize};
use url::Url;

use super::api_url;

/// A resource in the PokeAPI. Types implementing this trait can be automatically looked up by name/id
/// and paginated over.
pub trait ApiResource: Debug + Clone + PartialEq + Eq + for<'de> Deserialize<'de> + Serialize {
    /// The base URL for this API resource type
    fn base_url() -> Url;
}

/// A page of a resource list. See the [`NamedApiResourceList`](https://pokeapi.co/docs/v2#named) type.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Page<T: ApiResource> {
    /// The total number of resources available from this API.
    pub count: usize,
    /// The URL for the next page in the list.
    pub next: Option<Url>,
    /// The URL for the previous page in the list.
    pub previous: Option<Url>,
    /// A list of named API resources.
    #[serde(bound(deserialize = "Vec<NamedResource<T>>: Deserialize<'de>"))]
    pub results: Vec<NamedResource<T>>,
}

/// A named PokeAPI resource. This is typed to indicate what kind of API resource it points to.
/// See [`NamedAPIResource`](https://pokeapi.co/docs/v2#namedapiresource)/
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct NamedResource<T: ApiResource> {
    pub name: String,
    pub url: Url,
    /// Tells the compiler that this type acts like it points to a `T`
    #[serde(skip_serializing, default)]
    _typ: PhantomData<fn() -> T>,
}

/// A localized name for a resource. See [`Name`](https://pokeapi.co/docs/v2#name)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub name: String,
    pub language: NamedResource<Language>,
}

/// A language supported by the PokeAPI. See [`Language`](https://pokeapi.co/docs/v2#language)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Language {
    /// The identifier for this resource.
    pub id: usize,
    /// The name for this resource.
    pub name: String,
    /// Whether or not the games are published in this language.
    pub official: bool,
    /// The two-letter code of the country where this language is spoken. Note that it is not unique.
    pub iso639: String,
    /// The two-letter code of the language. Note that it is not unique.
    pub iso3166: String,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
}

/// A Pokemon. See [the API](https://pokeapi.co/docs/v2#pokemon).
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    /// The identifier for this resource.
    pub id: usize,
    /// The name for this resource.
    pub name: String,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: i32,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: bool,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub order: i32,
    /// The species this Pokémon belongs to.
    pub species: NamedResource<PokemonSpecies>,
}

/// A species of Pokemon. See [the API](https://pokeapi.co/docs/v2#pokemonspecies)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PokemonSpecies {
    /// The identifier for this resource.
    pub id: usize,
    /// The name for this resource.
    pub name: String,
    /// The order in which species should be sorted. Based on National Dex order, except families are grouped together
    /// and sorted by stage.
    pub order: i32,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: i8,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: u8,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_happiness: u8,
    /// Whether or not this is a baby Pokémon.
    pub is_baby: bool,
    /// Whether or not this is a legendary Pokémon.
    pub is_legendary: bool,
    /// Whether or not this is a mythical Pokémon.
    pub is_mythical: bool,
    /// Initial hatch counter: one must walk 255 × (hatch_counter + 1) steps before this Pokémon's egg hatches, unless
    /// utilizing bonuses like Flame Body's.
    pub hatch_counter: i32,
    /// Whether or not this Pokémon has visual gender differences.
    pub has_gender_differences: bool,
    /// Whether or not this Pokémon has multiple forms and can switch between them.
    pub forms_switchable: bool,
    // The rate at which this Pokémon species gains levels.
    // pub growth_rate: NamedResource,
    /// The name of this resource listed in different languages.
    pub names: Vec<Name>,
    /// A list of flavor text entries for this Pokémon species.
    pub flavor_text_entries: Vec<FlavorText>,
    /// Descriptions of different forms Pokémon take on within the Pokémon species.
    pub form_descriptions: Vec<Description>,
}

/// See [`FlavorText`](https://pokeapi.co/docs/v2#flavortext)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FlavorText {
    /// The localized flavor text for an API resource in a specific language.
    pub flavor_text: String,
    /// The language this name is in.
    pub language: NamedResource<Language>,
    // not including version field
}

/// See [`Description`](https://pokeapi.co/docs/v2#description)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Description {
    /// The localized description for an API resource in a specific language.
    pub description: String,
    /// The language this name is in.
    pub language: NamedResource<Language>,
}

impl ApiResource for Language {
    fn base_url() -> Url {
        api_url("language/")
    }
}

impl ApiResource for Pokemon {
    fn base_url() -> Url {
        api_url("pokemon/")
    }
}

impl PartialOrd for Pokemon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for Pokemon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl ApiResource for PokemonSpecies {
    fn base_url() -> Url {
        api_url("pokemon-species/")
    }
}
