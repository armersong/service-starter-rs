use crate::prelude::errcode::RC_INVALID_TOKEN;
use crate::Result;
use crate::{Config, ErrorKind};
use common::util::now_in_milliseconds;
use common::RedisCache;
use serde::{Deserialize, Serialize};

pub const SESSION_KEY_PREFIX_CLEANER: &str = "clr";
pub const SESSION_KEY_PREFIX_VENDOR: &str = "vdr";
pub const SESSION_KEY_PREFIX_ADMIN: &str = "admin";

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub sqn: u32,
    /// create time in sec
    pub ctime: u64,
    pub others: Option<String>,
}

impl Session {
    pub async fn create(
        token: &str,
        key_prefix: &str,
        sqn: u32,
        others: Option<String>,
    ) -> Result<Session> {
        let ses = Session { sqn: sqn, ctime: now_in_milliseconds() / 1000, others };
        //save session
        RedisCache::inst().set_obj(token, &ses, Some(Config::get().session.expire_time)).await?;

        //set relattion: sqn : token
        RedisCache::inst()
            .set(
                Self::make_cache_key(key_prefix, sqn).as_str(),
                token,
                Some(Config::get().session.expire_time),
            )
            .await?;
        Ok(ses)
    }

    pub async fn clean_relation(key_prefix: &str, sqn: u32) -> Result<()> {
        let key = Self::make_cache_key(key_prefix, sqn);
        //remove old token
        RedisCache::inst().remove(key.as_str()).await?;
        Ok(())
    }

    pub async fn load(token: &str) -> Result<Session> {
        match RedisCache::inst().get_obj::<Session>(token).await? {
            Some(mut sess) => {
                let now = now_in_milliseconds() / 1000;
                if sess.ctime + Config::get().session.expire_time as u64 / 2 >= now {
                    sess.ctime = now;
                    sess.save(token).await?;
                }
                Ok(sess)
            }
            None => {
                warn!("token {} invalid or expired!", token);
                Err(ErrorKind::Custom(RC_INVALID_TOKEN, "Token失效或非法!".to_string()).into())
            }
        }
    }

    pub async fn save(&self, token: &str) -> Result<()> {
        RedisCache::inst().set_obj(token, self, Some(Config::get().session.expire_time)).await
    }

    pub async fn remove(token: &str, prefix: &str) -> Result<()> {
        if let Some(ref sess) = RedisCache::inst().get_obj::<Session>(token).await? {
            RedisCache::inst().remove(token).await?;
            RedisCache::inst().remove(Self::make_cache_key(prefix, sess.sqn).as_str()).await?;
        } else {
            warn!("token {} invalid or expired!", token);
            return Err(ErrorKind::Custom(1, "Token失效或非法!".to_string()).into());
        }
        Ok(())
    }

    fn make_cache_key(prefix: &str, sqn: u32) -> String {
        format!("{}/{}", prefix, sqn)
    }
}
