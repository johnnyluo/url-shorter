use crate::models::UrlMeta;
use redis::Commands;
use serde_json;

#[derive(Clone)]
pub struct Redis {
    client: redis::Client,
}

impl Redis {
    pub fn new() -> Self {
        let connection_string = Self::get_connection_string();
        let client = Self::create_pool(&connection_string);
        client.get_connection().unwrap();
        log::info!("connected to redis at {} successfully.", connection_string);
        Self { client }
    }

    fn create_pool(connection_string: &str) -> redis::Client {
        redis::Client::open(connection_string).unwrap()
    }

    fn get_connection_string() -> String {
        let host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
        let password = std::env::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_string());
        format!("redis://:{}@{}:{}", password, host, port)
    }

    /// get key from redis
    pub fn get_key(&self, key: &str) -> Result<UrlMeta, redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let exists: bool = conn.exists(key).unwrap();
        if !exists {
            return Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "key not found",
            )));
        }
        let value_json: String = conn.get(key).unwrap();
        let value: UrlMeta = serde_json::from_str(&value_json).unwrap();
        Ok(value)
    }
    pub fn set_key(&self, key: &str, value: &UrlMeta) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let value_json = serde_json::to_string(value).unwrap();
        let _: () = conn.set(key, value_json).unwrap();
        Ok(())
    }
    pub fn delete_key(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let _: usize = conn.del(key).unwrap();
        Ok(())
    }
}
