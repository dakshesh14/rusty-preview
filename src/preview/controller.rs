use std::{sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    Json,
};

use crate::config::state::AppState;

use super::{
    cache_repository::CacheRepository,
    model::{MetaDataResponse, PreviewParams},
    service::fetch_metadata,
};

pub async fn fetch_link_preview(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PreviewParams>,
) -> Json<MetaDataResponse> {
    let url = params.url.as_str();

    match &*state.cache_pool {
        Some(client) => {
            let cache_repo = CacheRepository::new(client);
            if let Some(metadata) = cache_repo.get_metadata(url).await {
                Json(metadata)
            } else {
                let metadata = fetch_metadata(url).await.unwrap();

                cache_repo
                    .set_metadata(&metadata, Duration::from_secs(10 * 60))
                    .await
                    .unwrap();

                Json(MetaDataResponse::from(metadata))
            }
        }
        None => {
            let metadata = fetch_metadata(url).await.unwrap();
            Json(MetaDataResponse::from(metadata))
        }
    }
}
