use axum::{routing::get, Router};
use std::sync::Arc;

use crate::{config::state::AppState, preview::controller::fetch_link_preview};

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new().route("/preview", get(fetch_link_preview))
}
