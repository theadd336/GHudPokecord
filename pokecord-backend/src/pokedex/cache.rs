use std::{fmt::Write, io::ErrorKind, path::PathBuf};

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
    pub fn new() -> Cache {
        Cache::with_dir(".pokecache")
    }

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
    #[tracing::instrument]
    pub async fn get(&mut self, key: &CacheKey) -> Result<Option<Entry>, Error> {
        let data = match fs::read(&key.0).await {
            Ok(data) => data,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };

        let entry: Entry = bincode::deserialize(&data)?;
        tracing::debug!("Read {} bytes from cache", entry.body.len());
        Ok(Some(entry))
    }

    /// Update a raw cache entry
    #[tracing::instrument(skip(value))]
    pub async fn put(&mut self, key: &CacheKey, value: &Entry) -> Result<(), Error> {
        tracing::debug!("Caching {} bytes", value.body.len());
        let data = bincode::serialize(value)?;
        fs::create_dir_all(&self.dir).await?;
        fs::write(&key.0, data).await?;
        Ok(())
    }
}

impl Entry {
    pub fn new(body: Bytes, cache_policy: CachePolicy) -> Entry {
        Entry { cache_policy, body }
    }
}
