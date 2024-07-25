use axum::{
    routing::{get, post},
    Json, Router,
};
use lib_surrealdb::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

use crate::middlewares::auth::CtxW;
use crate::routes::Result;

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/users", get(api_get_users_handler))
        .with_state(mm)
}

// region:    --- Users
async fn api_get_users_handler(ctxw: CtxW) -> Result<Json<Value>> {
    debug!("{:<12} - api_get_users_handler", "HANLDER");
    let ctx = ctxw.0;
    debug!("{:<12} - api_get_users_handler {:?}", "HANDLER", ctx);

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
