use axum::{extract::Query, Json};

use super::{
    model::{MetaDataResponse, PreviewParams},
    service::fetch_metadata,
};

pub async fn fetch_link_preview(Query(params): Query<PreviewParams>) -> Json<MetaDataResponse> {
    let url = params.url.as_str();
    let metadata = fetch_metadata(url).await.unwrap();
    Json(metadata)
}
