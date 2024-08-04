use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use lib_surrealdb::model::{
    users::{
        bmc::UsersBmc, UsersForCreate, UsersForDelete, UsersForUpdate, UsersForUpdateByAdmin,
        UsersGet, UsersRecord,
    },
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

#[derive(Debug, Deserialize)]
struct UsersForUpdatePayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub lastname: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UsersForUpdateByAdminPayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub lastname: Option<String>,
    pub image: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize)]
struct PageParams {
    user_id: String,
}

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/users", get(list_users_handler).post(create_user_handler))
        .route(
            "/users/:user_id",
            get(get_users_handler)
                .put(update_user_handler)
                .delete(delete_user_handler),
        )
        .route("/users/:user_id/password", put(update_pwd_user_handler))
        .route(
            "/users/:user_id/update_by_admin",
            put(update_user_by_admin_handler),
        )
        .with_state(mm)
}

// region:    --- Users
async fn get_users_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { user_id }): Path<PageParams>,
) -> Result<Json<Value>> {
    debug!("{:<12} - get_users_handler", "HANLDER");
    let ctx = ctxw.0;
    let user_id_from_ctx = ctx.user_id().ok_or(Error::UserIdInCtxNotFound)?;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !(is_authorized || user_id_from_ctx == &user_id) {
        return Err(Error::YourUserNotAuthorize);
    }

    let users = UsersBmc::get::<UsersGet>(&ctx, &mm, &user_id)
        .await?
        .ok_or(Error::DataNotFound)?;

    // -- Create the success body.
    let body = Json(json!(users));

    Ok(body)
}

async fn list_users_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_users_handler", "HANLDER");
    let ctx = ctxw.0;
    debug!("{:<12} - list_users_handler {:?}", "HANDLER", ctx);

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
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

async fn create_user_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Json(payload): Json<UsersForCreatePayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - create_user_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
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

async fn delete_user_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { user_id }): Path<PageParams>,
) -> Result<StatusCode> {
    debug!("{:<12} - delete_user_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    let user_for_delete = UsersForDelete {
        deleted_by: ctx.user_id_thing().ok_or(Error::UserIdInCtxNotFound)?,
    };

    let _ = UsersBmc::delete(&ctx, &mm, &user_id, user_for_delete).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn update_user_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { user_id }): Path<PageParams>,
    Json(payload): Json<UsersForUpdatePayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - update_user_handler", "HANDLER");
    let ctx = ctxw.0;
    let user_id_from_ctx = ctx.user_id().ok_or(Error::UserIdInCtxNotFound)?;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !(is_authorized || user_id_from_ctx == &user_id) {
        return Err(Error::YourUserNotAuthorize);
    }

    let user_for_update = UsersForUpdate {
        username: payload.username,
        email: payload.email,
        title: payload.title,
        firstname: payload.firstname,
        middlename: payload.middlename,
        lastname: payload.lastname,
        image: payload.image,
    };

    let user_record = UsersBmc::update::<UsersRecord>(&ctx, &mm, &user_id, user_for_update).await?;

    // -- Create the success body.
    let body = Json(json!(user_record));

    Ok((StatusCode::OK, body))
}

async fn update_user_by_admin_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { user_id }): Path<PageParams>,
    Json(payload): Json<UsersForUpdateByAdminPayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - update_user_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    let user_for_update = UsersForUpdateByAdmin {
        username: payload.username,
        email: payload.email,
        title: payload.title,
        firstname: payload.firstname,
        middlename: payload.middlename,
        lastname: payload.lastname,
        image: payload.image,
        role: payload.role,
    };

    let user_record =
        UsersBmc::update_by_admin::<UsersRecord>(&ctx, &mm, &user_id, user_for_update).await?;

    // -- Create the success body.
    let body = Json(json!(user_record));

    Ok((StatusCode::OK, body))
}

async fn update_pwd_user_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { user_id }): Path<PageParams>,
) -> Result<(StatusCode, Json<Value>)> {
    todo!()
}

// endregion: --- Users
