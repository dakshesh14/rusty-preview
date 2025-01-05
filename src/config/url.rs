use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

use crate::preview::url::get_routes as get_preview_routes;

pub fn get_routes() -> Router<Arc<PgPool>> {
    Router::new().merge(get_preview_routes())
}
