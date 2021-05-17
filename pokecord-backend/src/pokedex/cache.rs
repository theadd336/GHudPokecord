use std::{fmt::{self, Display, Write}, io::ErrorKind, path::PathBuf};

use bytes::Bytes;
use http_cache_semantics::CachePolicy;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_384};
use tokio::fs;
use url::Url;

use super::Error;

/// Filesystem-backed, URL-addressed cache for HTTP requests.
///
/// ## TODOs:
/// * In-memory cache on top of filesystem cache
#[derive(Debug)]
pub struct Cache {
    dir: PathBuf,
}

/// Version prefix included in filenames to allow backwards-incompatible changes to the on-disk cache layout.
const VERSION: &str = "v2";

#[derive(Debug, PartialEq, Eq)]
pub struct CacheKey(PathBuf);

#[derive(Deserialize, Serialize)]
pub struct Entry {
    /// Policy this entry was cached with
    pub cache_policy: CachePolicy,
    /// The cached response body
    pub body: Bytes,
}

impl Cache {
    /// Create a new cache that stores files in the default directory.
    pub fn new() -> Cache {
        Cache::with_dir(".pokecache")
    }

    /// Create a new cache that stores files in the given directory.
    pub fn with_dir<P: Into<PathBuf>>(dir: P) -> Cache {
        Cache { dir: dir.into() }
    }

    /// Builds a cache key from a URL. This key can be used to retrieve and update the cache's stored result for that URL.
    pub fn cache_key(&self, url: &Url) -> CacheKey {
        let hash = Sha3_384::digest(url.as_str().as_bytes());
        let mut name = String::new();
        for byte in hash {
            let _ = write!(&mut name, "{:02x}", byte);
        }
        CacheKey(self.dir.join(VERSION).join(name))
    }

    /// Retrieve a raw cache entry
    pub async fn get(&mut self, key: &CacheKey) -> Result<Option<Entry>, Error> {
        log::debug!("Fetching {} from cache", key);
        let data = match fs::read(&key.0).await {
            Ok(data) => data,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                log::trace!("Cache miss for {}", key);
                return Ok(None)
            },
            Err(err) => {
                log::warn!("Cache read error for {}: {}", key, err);
                return Err(err.into());
            },
        };

        let entry: Entry = bincode::deserialize(&data)?;
        Ok(Some(entry))
    }

    /// Update a raw cache entry
    pub async fn put(&mut self, key: &CacheKey, value: &Entry) -> Result<(), Error> {
        log::debug!("Writing {}-byte cache entry for {}", value.body.len(), key);
        let data = bincode::serialize(value)?;
        fs::create_dir_all(&self.dir).await?;
        fs::write(&key.0, data).await?;
        Ok(())
    }
}

impl fmt::Display for CacheKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl Entry {
    pub fn new(body: Bytes, cache_policy: CachePolicy) -> Entry {
        Entry { cache_policy, body }
    }
}
