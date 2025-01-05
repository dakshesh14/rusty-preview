use super::model::{MetaData, MetaDataResponse};
use redis::{AsyncCommands, Client as RedisClient};
use std::time::Duration;

pub struct CacheRepository<'a> {
    client: &'a RedisClient,
}

impl<'a> CacheRepository<'a> {
    pub fn new(client: &'a RedisClient) -> Self {
        Self { client }
    }

    pub async fn get_metadata(&self, url: &str) -> Option<MetaDataResponse> {
        let mut conn = self.client.get_multiplexed_async_connection().await.ok()?;
        let result: Option<String> = conn.get(url).await.ok()?;
        result.and_then(|data| serde_json::from_str(&data).ok())
    }

    pub async fn set_metadata(&self, metadata: &MetaData, ttl: Duration) -> redis::RedisResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let json = serde_json::to_string(metadata).unwrap();
        conn.set_ex(metadata.link.as_str(), json, ttl.as_secs() as u64)
            .await
    }
}
