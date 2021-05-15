use std::time::SystemTime;

use http_cache_semantics::{AfterResponse, BeforeRequest, CacheOptions, CachePolicy};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

mod api_models;
mod cache;
mod error;

pub use api_models::*;
pub use error::Error;

use crate::pokedex::cache::Entry;

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
    pub async fn get_by_name<T: ApiResource + Serialize + DeserializeOwned>(
        &mut self,
        name: &str,
    ) -> Result<T, Error> {
        let url = T::base_url().join(name).expect("Malformed resource name");
        self.get(url).await
    }

    /// Get an API resource using a reference from another resource.
    pub async fn get_by_ref<T: ApiResource + Serialize + DeserializeOwned>(
        &mut self,
        reference: &NamedResource<T>,
    ) -> Result<T, Error> {
        self.get(reference.url.clone()).await
    }

    /// Read an entire resource list. This may be expensive.
    pub async fn list<T: ApiResource + Serialize + DeserializeOwned>(
        &mut self,
    ) -> Result<Vec<NamedResource<T>>, Error> {
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
    async fn page_list<T: ApiResource + Serialize + DeserializeOwned>(
        &mut self,
        cursor: Cursor,
    ) -> Result<(Vec<NamedResource<T>>, Option<Cursor>), Error> {
        let url = match cursor {
            Cursor::Begin {
                mut url,
                offset,
                limit,
            } => {
                url.query_pairs_mut()
                    .append_pair("offset", &offset.to_string())
                    .append_pair("limit", &limit.to_string());
                url
            }
            Cursor::Next(url) => url,
        };

        let page: Page<T> = self.get(url).await?;

        let next_cursor = page.next.map(Cursor::Next);
        Ok((page.results, next_cursor))
    }

    #[tracing::instrument(skip(self), fields(url = %url))]
    async fn get<T: Serialize + DeserializeOwned>(&mut self, url: Url) -> Result<T, Error> {
        let cache_key = self.cache.cache_key(&url);
        let mut req = self.client.get(url.clone()).build()?;

        match self.cache.get(&cache_key).await {
            Ok(Some(entry)) => {
                let now = SystemTime::now();
                tracing::debug!(age = ?entry.cache_policy.age(now), ttl = ?entry.cache_policy.time_to_live(now), "Found cache entry");
                match entry.cache_policy.before_request(&req, SystemTime::now()) {
                    // Cache was up to date, so use the value stored there
                    BeforeRequest::Fresh(_) => {
                        tracing::debug!("Cached body is fresh");
                        Ok(serde_json::from_slice::<T>(&entry.body)?)
                    }
                    BeforeRequest::Stale { request, .. } => {
                        tracing::debug!("Cached body is stale, will revalidate");
                        // Cache is stale, send a revalidation request - we only need to copy the revalidation headers
                        *req.headers_mut() = request.headers;

                        // Send the revalidation request - this is different from sending an uncached request because
                        // the server may respond with a 304 not modified, in which case we still use the cached body.
                        // .try_clone().unwrap() is safe because there's no request body
                        let res = self
                            .client
                            .execute(req.try_clone().unwrap())
                            .await?
                            .error_for_status()?;
                        let (policy, bytes) =
                            match entry
                                .cache_policy
                                .after_response(&req, &res, SystemTime::now())
                            {
                                AfterResponse::NotModified(policy, _) => {
                                    tracing::debug!("Cached body is up to date");
                                    (policy, entry.body)
                                }
                                AfterResponse::Modified(policy, _) => {
                                    tracing::debug!("Cached body was modified");
                                    (policy, res.bytes().await?)
                                }
                            };

                        let value = serde_json::from_slice(bytes.as_ref())?;

                        // Update the cache, but don't let this fail the whole request
                        if policy.is_storable() {
                            if let Err(err) =
                                self.cache.put(&cache_key, &Entry::new(bytes, policy)).await
                            {
                                tracing::warn!(%err, "Cache update failed");
                            }
                        } else {
                            tracing::debug!("Request is not cacheable");
                        }

                        Ok(value)
                    }
                }
            }
            Ok(None) | Err(_) => {
                // There's nothing in cache (or accessing it failed), we have to make a new request
                tracing::debug!("No cache entry");

                // .try_clone().unwrap() is safe because there's no request body
                let res = self
                    .client
                    .execute(req.try_clone().unwrap())
                    .await?
                    .error_for_status()?;

                // We _mostly_ want the defaults, but this is a private cache, not a shared one (i.e. a proxy), so we
                // can cache more things.
                let opts = CacheOptions {
                    shared: false,
                    ..Default::default()
                };
                let policy = CachePolicy::new_options(&req, &res, SystemTime::now(), opts);

                let bytes = res.bytes().await?;
                let value = serde_json::from_slice(bytes.as_ref())?;

                if policy.is_storable() {
                    if let Err(err) = self.cache.put(&cache_key, &Entry::new(bytes, policy)).await {
                        tracing::warn!(%err, "Cache update failed");
                    }
                } else {
                    tracing::debug!("Request is not cacheable");
                }

                Ok(value)
            }
        }
    }
}

fn api_url(path: &str) -> Url {
    Url::parse(API_BASE)
        .unwrap()
        .join(path)
        .expect("Invalid API URL")
}
