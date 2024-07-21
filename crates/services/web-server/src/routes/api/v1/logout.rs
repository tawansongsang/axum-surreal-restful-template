use axum::{routing::post, Json, Router};
use lib_surrealdb::model::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::Result;

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/logout", post(api_logout_handler))
        .with_state(mm)
}

// region:    --- Logout
async fn api_logout_handler(
    // cookies: Cookies,
    Json(payload): Json<LogoutPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_logout_handler", "HANLDER");

    let should_logout = payload.logout;

    // if should_logout {
    //     remove_token_cookie(&cookies)?;
    // }

    // -- Create the success body.
    let body = Json(json!({
        "resutl": {
            "logout": should_logout
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoutPayload {
    logout: bool,
}
// endregion: --- Logout
