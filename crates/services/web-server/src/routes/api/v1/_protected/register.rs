use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use lib_surrealdb::model::{
    users::{bmc::UsersBmc, UsersForCreate},
    ModelManager,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::{
    middlewares::auth::CtxW,
    routes::{error::Result, Error},
};

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
    ctxw: CtxW,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - api_register_handler", "HANLDER");
    // let root_ctx = Ctx::root_ctx();
    let ctx = ctxw.0;

    // check authorize admin
    let user_id = UsersBmc::is_admin(&ctx, &mm).await?;
    if user_id == false {
        return Err(Error::YourUserNotAuthorize);
    }

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

    let user_info_record = UsersBmc::create(&ctx, &mm, user_info_for_create).await?;

    let body = Json(json!(user_info_record));

    Ok((StatusCode::CREATED, body))
}
