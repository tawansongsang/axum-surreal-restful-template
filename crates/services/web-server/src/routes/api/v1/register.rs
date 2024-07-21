use axum::{extract::State, routing::post, Json, Router};
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        users::{bmc::UsersBmc, UsersForCreate},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::error::Result;

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    title: String,
    firstname: String,
    middlename: Option<String>,
    lastname: String,
    password: String,
}

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/register", post(api_register_handler))
        .with_state(mm)
}

async fn api_register_handler(
    State(mm): State<ModelManager>,
    // _cookies: Cookies,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_register_handler", "HANLDER");
    let root_ctx = Ctx::root_ctx();

    let RegisterPayload {
        username,
        email,
        title,
        firstname,
        middlename,
        lastname,
        password,
    } = payload;

    let user_info_for_create = UsersForCreate {
        username,
        email,
        title,
        firstname,
        middlename,
        lastname,
        password,
    };

    let _user_info_record = UsersBmc::create(&root_ctx, &mm, user_info_for_create).await?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
