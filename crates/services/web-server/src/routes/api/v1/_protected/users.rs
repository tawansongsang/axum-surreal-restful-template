use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lib_surrealdb::model::{
    users::{bmc::UsersBmc, UsersForCreate, UsersGet, UsersRecord},
    ModelManager,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::{Error, Result};
use crate::{middlewares::auth::CtxW, params::PaginationParams};

#[derive(Debug, Deserialize)]
struct UsersForCreatePayload {
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
        .route(
            "/users",
            get(api_get_users_handler).post(api_create_user_handler),
        )
        .with_state(mm)
}

// region:    --- Users
async fn api_get_users_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_get_users_handler", "HANLDER");
    let ctx = ctxw.0;
    debug!("{:<12} - api_get_users_handler {:?}", "HANDLER", ctx);

    // check authorize admin
    let user_id = UsersBmc::is_admin(&ctx, &mm).await?;
    if user_id == false {
        return Err(Error::YourUserNotAuthorize);
    }

    // -- Get limit and offset from Query Params
    let limit = params.limit;
    let offset = params.offset;
    let order = params.order;

    let users = UsersBmc::list::<UsersGet>(&ctx, &mm, limit, offset, order).await?;

    // -- Create the success body.
    let body = Json(json!(users));

    Ok(body)
}

// TODO: implement create user handler
async fn api_create_user_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Json(payload): Json<UsersForCreatePayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - api_create_user_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let user_id = UsersBmc::is_admin(&ctx, &mm).await?;
    if user_id == false {
        return Err(Error::YourUserNotAuthorize);
    }

    let UsersForCreatePayload {
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

    let user_record = UsersBmc::create(&ctx, &mm, user_info_for_create, true).await?;

    // -- Create the success body.
    let body = Json(json!(user_record));

    Ok((StatusCode::CREATED, body))
}

// endregion: --- Users
