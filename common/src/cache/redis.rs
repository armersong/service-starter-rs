use crate::Result;
use redis_async_pool::deadpool::managed::Timeouts;
use redis_async_pool::{RedisConnectionManager, RedisPool};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

pub struct RedisCache(Arc<RedisCacheInner>);

impl RedisCache {
    pub fn new(url: &str, num: usize) -> Result<Self> {
        let pool = RedisPool::new(
            RedisConnectionManager::new(credis::Client::open(url)?, true, None),
            num,
        );
        Ok(Self(Arc::new(RedisCacheInner {
            pool,
            timeout: 3000,
        })))
    }

    /// Only call once!
    pub fn init_singleton_once(url: &str, num: usize) -> Result<()> {
        unsafe {
            if REDIS_CACHE.is_none() {
                REDIS_CACHE = Some(Self::new(url, num)?);
            } else {
                panic!("RedisCache init only once!");
            }
        }
        Ok(())
    }

    /// call init_singleton_once first!
    pub fn inst() -> &'static Self {
        unsafe {
            if let Some(ref cache) = REDIS_CACHE {
                cache
            } else {
                panic!("RedisCache not inited!");
            }
        }
    }

    pub async fn set<T: AsRef<str>>(&self, key: &str, val: T, expire: Option<u32>) -> Result<()> {
        let mut con = self.0.pool.timeout_get(&self.get_timeout()).await?;
        credis::cmd("SET")
            .arg(key)
            .arg(val.as_ref())
            .query_async(con.as_mut())
            .await?;
        if let Some(d) = expire {
            credis::cmd("EXPIRE")
                .arg(key)
                .arg(d)
                .query_async(con.as_mut())
                .await?;
        }
        Ok(())
    }

    pub async fn set_obj<T: Serialize>(
        &self,
        key: &str,
        val: &T,
        expire: Option<u32>,
    ) -> Result<()> {
        let s = serde_json::to_string(val)?;
        Ok(self.set(key, s, expire).await?)
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut con = self.0.pool.timeout_get(&self.get_timeout()).await?;
        let res = credis::cmd("GET")
            .arg(key)
            .query_async(con.as_mut())
            .await?;
        Ok(res)
    }

    pub async fn get_obj<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        Ok(match self.get(key).await? {
            Some(s) => Some(serde_json::from_reader::<_, T>(Cursor::new(Vec::from(s)))?),
            None => None,
        })
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut con = self.0.pool.timeout_get(&self.get_timeout()).await?;
        credis::cmd("DEL")
            .arg(key)
            .query_async(con.as_mut())
            .await?;
        Ok(())
    }

    fn get_timeout(&self) -> Timeouts {
        Timeouts {
            wait: Some(Duration::from_millis(self.0.timeout as u64)),
            create: Some(Duration::from_millis(self.0.timeout as u64)),
            recycle: Some(Duration::from_millis(self.0.timeout as u64)),
        }
    }
}

impl Clone for RedisCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct RedisCacheInner {
    pool: RedisPool,
    timeout: u32,
}

static mut REDIS_CACHE: Option<RedisCache> = None;

#[cfg(test)]
mod tests {
    use crate::RedisCache;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[test]
    fn test_redis() {
        futures_lite::future::block_on(async {
            RedisCache::init_singleton_once("redis://localhost", 5).expect("connect redis failed");
            assert!(RedisCache::inst().remove("redis").await.is_ok());
            assert!(RedisCache::inst()
                .set("redis", "ok".to_string(), Some(1))
                .await
                .is_ok());
            assert_eq!(
                RedisCache::inst().get("redis").await.expect("get failed"),
                Some("ok".to_string())
            );
            async_io::Timer::after(Duration::from_secs(1)).await;
            assert_eq!(
                RedisCache::inst().get("redis").await.expect("get failed"),
                None
            );
        });
    }

    #[test]
    fn test_redis_obj() {
        futures_lite::future::block_on(async {
            RedisCache::init_singleton_once("redis://localhost", 5).expect("connect redis failed");
            #[derive(Serialize, Deserialize)]
            struct Object {
                x: i32,
                y: String,
            }
            let v = Object {
                x: 1,
                y: "ok".to_string(),
            };
            assert!(RedisCache::inst().remove("obj").await.is_ok());
            assert!(RedisCache::inst().set_obj("obj", &v, Some(1)).await.is_ok());
            let v1 = RedisCache::inst()
                .get_obj::<Object>("obj")
                .await
                .expect("get failed")
                .expect("not found!");
            assert_eq!(v1.x, v.x);
            assert_eq!(v1.y, v.y);
        });
    }
}
