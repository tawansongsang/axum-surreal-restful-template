use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    healthy: bool,
}

pub fn route() -> Router {
    let routes = Router::new();
    routes.route("/health", get(health))
}

pub(crate) async fn health() -> impl IntoResponse {
    let health = Health { healthy: true };

    (StatusCode::OK, Json(health))
}
