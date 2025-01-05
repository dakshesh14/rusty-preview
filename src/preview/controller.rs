use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    Json,
};

use super::{
    cache_repository::{CacheRepository, RedisRepository},
    model::{MetaDataResponse, PreviewParams},
    service::fetch_metadata,
};
use crate::config::state::AppState;

pub async fn fetch_link_preview(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PreviewParams>,
) -> Json<MetaDataResponse> {
    let url = params.url.as_str();

    match &*state.cache_pool {
        Some(cache_client) => {
            let cache_repo = RedisRepository::builder()
                .with_client(Arc::new(cache_client.clone()))
                .build()
                .expect("Failed to build cache repository");

            if let Some(metadata) = cache_repo
                .get_metadata(url)
                .await
                .expect("Failed to get metadata from cache")
            {
                return Json(metadata);
            }

            let metadata = fetch_metadata(url).await.expect("Failed to fetch metadata");

            cache_repo
                .set_metadata(&metadata, Duration::from_secs(10 * 60))
                .await
                .expect("Failed to store metadata in cache");

            Json(MetaDataResponse::from(&metadata))
        }
        None => {
            let metadata = fetch_metadata(url).await.expect("Failed to fetch metadata");
            Json(MetaDataResponse::from(&metadata))
        }
    }
}
