use std::time::Duration;

use crate::error::{Error, Result};
use crate::service::ICacheService;
use futures_util::future::BoxFuture;
use log::error;
use redis::aio::Connection;
use redis::RedisResult;

///Redis Cache service
#[derive(Debug, Clone)]
pub struct RedisService {
    pub client: redis::Client,
    url: String,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        println!("[ruoyi_rust] conncect redis ({})...", url);
        let client = redis::Client::open(url).unwrap();
        println!("[ruoyi_rust] conncect redis success!");
        Self { client, url: url.to_string() }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connection fail:{}", conn.err().unwrap());
            error!("{}, {}", self.url, err);
            return Err(crate::error::Error::from(err));
        }
        return Ok(conn?);
    }
}

impl ICacheService for RedisService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            return self.set_string_ex(&k, &v, None).await;
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Option<String>> =
                redis::cmd("GET").arg(&[&k]).query_async(&mut conn).await;
            return match result {
                Ok(v) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "RedisService get_string({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    ///set_string Automatically expire
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return if ex.is_none() {
                match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            } else {
                match redis::cmd("SET")
                    .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                    .query_async(&mut conn)
                    .await
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "RedisService set_string_ex fail:{}",
                        e.to_string()
                    ))),
                }
            };
        })
    }

    ///get time to live
    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService ttl fail:{}",
                    e.to_string()
                ))),
            };
        })
    }
    fn del(&self, k: &str) -> BoxFuture<Result<bool>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("DEL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService del fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    fn keys(&self, k: &str) -> BoxFuture<Result<Vec<String>>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("KEYS").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService del fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    fn hgetall(&self, key: &str) -> BoxFuture<Result<Vec<String>>> {
        let k = key.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("HGETALL").arg(&[k]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService HGETALL fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    fn expire(&self, k: &str, time_sec: i32) -> BoxFuture<Result<bool>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return match redis::cmd("EXPIRE").arg(k).arg(time_sec).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService del fail:{}",
                    e.to_string()
                ))),
            };
        })
    }

    //从在db插入
    fn hset(&self, k: &str, f: &str, v: &str) -> BoxFuture<Result<u64>> {
        let k = k.to_string();
        let f = f.to_string();
        let v = v.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let res: Result<()> = match redis::cmd("SELECT").arg("21").query_async(&mut conn).await {
                Ok(v) => Ok(v) ,
                Err(e) => Err(Error::from(format!(
                    "RedisService hset fail:{}",
                    e.to_string()
                ))),
            };
            match redis::cmd("HSET").arg(&[k, f, v]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService hset fail:{}",
                    e.to_string()
                ))),
            }
        })
    }
    fn select(&self, db: &str) -> BoxFuture<Result<()>> {
        let db = db.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            match redis::cmd("SELECT").arg(&[db]).query_async(&mut conn).await {
                Ok(v) => Ok(v),
                Err(e) => Err(Error::from(format!(
                    "RedisService select fail:{}",
                    e.to_string()
                ))),
            }
        })
    }
}
