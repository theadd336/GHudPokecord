use std::fmt::Write;
use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize};
use sha3::{Sha3_384, Digest};
use tokio::{fs::{self, File}, io::{AsyncReadExt, AsyncWriteExt}};
use url::Url;

use super::Error;

/// Filesystem-backed, URL-addressed cache for static API resources. The caching scheme is fairly simple,
/// since PokeAPI is read-only and we assume that Pokemon data doesn't change often.
///
/// ## TODOs:
/// * Cache invalidation
/// * In-memory cache on top of filesystem cache
pub struct Cache {
    dir: PathBuf,
    /// Shared buffer to reuse when reading from cache
    buf: Vec<u8>,
}

pub struct CacheKey(PathBuf);

impl Cache {
    pub fn new() -> Cache {
        Cache::with_dir(".pokecache")
    }

    pub fn with_dir<P: Into<PathBuf>>(dir: P) -> Cache {
        Cache { dir: dir.into(), buf: Vec::new() }
    }

    /// Builds a cache key from a URL. This key can be used to retrieve and update the cache's stored result for that URL.
    pub fn cache_key(&self, url: &Url) -> CacheKey {
        let hash = Sha3_384::digest(url.as_str().as_bytes());
        let mut name = String::new();
        for byte in hash {
            let _ = write!(&mut name, "{:02x}", byte);
        }
        CacheKey(self.dir.join(name))
    }

    pub async fn get<T: DeserializeOwned>(&mut self, key: &CacheKey) -> Option<T> {
        let mut file = File::open(&key.0).await.ok()?;
        self.buf.clear();
        file.read_to_end(&mut self.buf).await.ok()?;
        // TODO: consider a more efficient binary format for the cache?
        serde_json::from_slice::<T>(&self.buf).ok()
    }

    pub async fn put<T: Serialize>(&mut self, key: &CacheKey, value: &T) -> Result<(), Error> {
        fs::create_dir_all(&self.dir).await?;
        let mut file = File::create(&key.0).await?;
        let data = serde_json::to_vec(value)?;
        file.write_all(&data).await?;
        Ok(())
    }
}