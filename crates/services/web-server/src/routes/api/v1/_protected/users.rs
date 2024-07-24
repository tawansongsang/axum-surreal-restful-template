use axum::{
    routing::{get, post},
    Json, Router,
};
use lib_surrealdb::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::Result;

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/users", get(api_get_users_handler))
        .with_state(mm)
}

// region:    --- Users
async fn api_get_users_handler() -> Result<Json<Value>> {
    debug!("{:<12} - api_get_users_handler", "HANLDER");

    // -- Create the success body.
    let body = Json(json!({
        "result": {
            "authorization": true
        }
    }));

    Ok(body)
}

async fn api_create_user_handler() -> Result<Json<Value>> {
    debug!("{:<12} - api_create_user_handler", "HANDLER");

    // -- Create the success body.
    let body = Json(json!({
        "result": {
            "authorization": true
        }
    }));

    Ok(body)
}

// endregion: --- Users
