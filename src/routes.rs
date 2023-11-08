pub mod contracts;
pub mod views;

use axum::{routing::{get, post},Router};

use crate::routes::views::{basic_view, redirect_to_view, create_slug_view, create_slug_variant_view};

pub fn api_routes() -> Router {
    Router::new()
        .route("/", get(basic_view))
        .route("/redirect/:slug", get(redirect_to_view))
        .route("/add/slug", post(create_slug_view))
        .route("/add/slug/variant", post(create_slug_variant_view))
}
