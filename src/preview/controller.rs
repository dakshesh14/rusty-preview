use axum::{extract::Query, Json};

use super::{
    model::{MetaData, PreviewParams},
    service::fetch_metadata,
};

pub async fn fetch_link_preview(Query(params): Query<PreviewParams>) -> Json<MetaData> {
    let url = params.url.as_str();
    let metadata = fetch_metadata(url).await.unwrap();
    Json(metadata)
}
