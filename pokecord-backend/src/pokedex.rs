use std::time::Duration;

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

mod cache;
mod error;
mod api_models;

pub use error::Error;
pub use api_models::*;

use self::cache::Cache;

const API_BASE: &str = "https://pokeapi.co/api/v2/";

pub struct Pokedex {
    client: Client,
    cache: Cache,
}

pub enum Cursor {
    Begin {
        url: Url,
        offset: usize,
        limit: usize,
    },
    Next(Url),
}

impl Pokedex {
    pub fn new() -> Pokedex {
        Pokedex {
            client: Client::new(),
            cache: Cache::new(),
        }
    }

    /// Get an API resource by name.
    pub async fn get_by_name<T: ApiResource + Serialize + DeserializeOwned>(&mut self, name: &str) -> Result<T, Error> {
        let url = T::base_url().join(name).expect("Malformed resource name");
        self.get(url).await
    }

    /// Get an API resource using a reference from another resource.
    pub async fn get_by_ref<T: ApiResource + Serialize + DeserializeOwned>(&mut self, reference: &NamedResource<T>) -> Result<T, Error> {
        self.get(reference.url.clone()).await
    }

    /// Read an entire resource list. This may be expensive.
    pub async fn list<T: ApiResource + Serialize + DeserializeOwned>(&mut self) -> Result<Vec<NamedResource<T>>, Error> {
        let mut acc = Vec::new();
        let mut cursor = Some(Cursor::Begin {
            url: T::base_url(),
            // Use a chunk size of 100
            limit: 100,
            offset: 0,
        });

        while let Some(c) = cursor {
            let (results, next_cursor) = self.page_list(c).await?;
            acc.extend(results.into_iter());
            cursor = next_cursor;
        }

        Ok(acc)
    }

    /// Paginate over a resource list. The cursor allows either starting at a particular offset or continuing
    /// from a previous request. Reverse pagination is currently not supported.
    async fn page_list<T: ApiResource + Serialize + DeserializeOwned>(&mut self, cursor: Cursor) -> Result<(Vec<NamedResource<T>>, Option<Cursor>), Error> {
        let url = match cursor {
            Cursor::Begin { mut url, offset, limit } => {
                url
                    .query_pairs_mut()
                    .append_pair("offset", &offset.to_string())
                    .append_pair("limit", &limit.to_string());
                url
            },
            Cursor::Next(url) => url
        };

        let page: Page<T> = self.get(url).await?;

        let next_cursor = page.next.map(Cursor::Next);
        Ok((page.results, next_cursor))
    }

    async fn get<T: Serialize + DeserializeOwned>(&mut self, url: Url) -> Result<T, Error> {
        let cache_key = self.cache.cache_key(&url);
        if let Some(cached) = self.cache.get(&cache_key).await {
            // TODO: log/track hit/miss rate
            Ok(cached)
        } else {
            let result = self.client.get(url).send().await?.error_for_status()?;
            let json = result.json().await?;

            if let Err(err) = self.cache.put(&cache_key, &json, Duration::from_secs(3600)).await {
                // TODO: log cache update error
                eprintln!("Cache update failed: {}", err);
            }

            Ok(json)
        }
    }
}

fn api_url(path: &str) -> Url {
    Url::parse(API_BASE).unwrap().join(path).expect("Invalid API URL")
}