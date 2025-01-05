use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::preview::controller::fetch_link_preview;

pub fn get_routes() -> Router<Arc<PgPool>> {
    Router::new().route("/preview", get(fetch_link_preview))
}
