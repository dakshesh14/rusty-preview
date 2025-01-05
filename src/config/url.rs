use axum::Router;
use std::sync::Arc;

use crate::preview::url::get_routes as get_preview_routes;

use super::state::AppState;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new().merge(get_preview_routes())
}
