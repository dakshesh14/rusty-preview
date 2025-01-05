use redis::Client as RedisClient;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub cache_pool: Arc<Option<RedisClient>>,
}
