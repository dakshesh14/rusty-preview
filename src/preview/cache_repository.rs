use async_trait::async_trait;
use redis::{AsyncCommands, Client as RedisClient};
use std::{sync::Arc, time::Duration};
use thiserror::Error;

use super::model::{MetaData, MetaDataResponse};

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, CacheError>;

#[async_trait]
pub trait CacheRepository {
    async fn get_metadata(&self, url: &str) -> Result<Option<MetaDataResponse>>;
    async fn set_metadata(&self, metadata: &MetaData, ttl: Duration) -> Result<()>;
}

pub struct RedisRepository {
    client: Arc<RedisClient>,
}

#[derive(Default)]
pub struct RedisRepositoryBuilder {
    client: Option<Arc<RedisClient>>,
}

impl RedisRepositoryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: Arc<RedisClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Result<RedisRepository> {
        let client = self
            .client
            .ok_or_else(|| CacheError::Other("Redis client is required".to_string()))?;

        Ok(RedisRepository { client })
    }
}

impl RedisRepository {
    pub fn builder() -> RedisRepositoryBuilder {
        RedisRepositoryBuilder::new()
    }

    async fn get_connection(&self) -> Result<impl AsyncCommands> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(CacheError::Redis)
    }
}

#[async_trait]
impl CacheRepository for RedisRepository {
    async fn get_metadata(&self, url: &str) -> Result<Option<MetaDataResponse>> {
        let mut conn = self.get_connection().await?;

        let result: Option<String> = conn.get(url).await.map_err(CacheError::Redis)?;

        match result {
            Some(data) => Ok(Some(
                serde_json::from_str(&data).map_err(CacheError::Serialization)?,
            )),
            None => Ok(None),
        }
    }

    async fn set_metadata(&self, metadata: &MetaData, ttl: Duration) -> Result<()> {
        let mut conn = self.get_connection().await?;

        let json = serde_json::to_string(metadata).map_err(CacheError::Serialization)?;

        conn.set_ex(metadata.link.as_str(), json, ttl.as_secs() as u64)
            .await
            .map_err(CacheError::Redis)
    }
}
