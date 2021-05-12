use std::{fmt::Write, time::{Duration, SystemTime}};
use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize, Deserialize};
use sha3::{Sha3_384, Digest};
use tokio::{fs::{self, File}, io::{AsyncReadExt, AsyncWriteExt}};
use url::Url;

use super::Error;

/// Filesystem-backed, URL-addressed cache for static API resources. The caching scheme is fairly simple,
/// since PokeAPI is read-only and we assume that Pokemon data doesn't change often.
///
/// ## TODOs:
/// * In-memory cache on top of filesystem cache
pub struct Cache {
    dir: PathBuf,
    /// Shared buffer to reuse when reading from cache
    buf: Vec<u8>,
}

/// Version prefix included in filenames to allow backwards-incompatible changes to the on-disk cache layout.
const VERSION: &str = "v1";

pub struct CacheKey(PathBuf);

#[derive(Deserialize, Serialize)]
struct Wrapper<T> {
    /// Date at which this cache entry expires, in milliseconds since the UNIX epoch.
    expiry_epoch_secs: u64,
    value: T,
}

impl Cache {
    pub fn new() -> Cache {
        Cache::with_dir(".pokecache")
    }

    pub fn with_dir<P: Into<PathBuf>>(dir: P) -> Cache {
        Cache { dir: dir.into().join(VERSION), buf: Vec::new() }
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
        self.buf.clear();
        Self::get_raw(key, &mut self.buf).await.ok()?;
        let wrapper: Wrapper<T> = bincode::deserialize(&self.buf).ok()?;
        wrapper.if_unexpired()
    }

    pub async fn put<T: Serialize>(&mut self, key: &CacheKey, value: &T, ttl: Duration) -> Result<(), Error> {
        let data = bincode::serialize(&Wrapper::with_ttl(value, ttl))?;
        self.put_raw(key, &data).await?;
        Ok(())
    }

    async fn put_raw(&mut self, key: &CacheKey, buf: &[u8]) -> Result<(), Error> {
        fs::create_dir_all(&self.dir).await?;
        let mut file = File::create(&key.0).await?;
        file.write_all(&buf).await?;
        Ok(())
    }

    async fn get_raw(key: &CacheKey, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut file = File::open(&key.0).await?;
        file.read_to_end(buf).await?;
        Ok(())
    }
}

impl <T> Wrapper<T> {
    fn new(value: T, expiry: SystemTime) -> Wrapper<T> {
        Wrapper {
            value,
            expiry_epoch_secs: expiry.duration_since(SystemTime::UNIX_EPOCH).expect("Invalid expiry").as_secs(),
        }
    }

    fn with_ttl(value: T, ttl: Duration) -> Wrapper<T> {
        Wrapper::new(value, SystemTime::now() + ttl)
    }

    fn expiry_time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(self.expiry_epoch_secs)
    }

    fn is_expired(&self) -> bool {
        self.expiry_time() >= SystemTime::now()
    }

    fn if_unexpired(self) -> Option<T> {
        if self.is_expired() {
            None
        } else {
            Some(self.value)
        }
    }
}