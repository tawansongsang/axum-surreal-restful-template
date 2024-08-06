use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use lib_surrealdb::model::{users::bmc::UsersBmc, ModelManager};
use serde::Deserialize;
use serde_json::Value;
use tracing::debug;

use crate::routes::{Error, Result};
use crate::{middlewares::auth::CtxW, params::PaginationParams};

#[derive(Deserialize)]
struct PageParams {
    task_id: String,
}

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks_handler).post(create_tasks_handler))
        .route(
            "/tasks/:task_id",
            get(get_tasks_handler)
                .put(update_tasks_handler)
                .delete(delete_tasks_handler),
        )
        .with_state(mm)
}

// region:    --- Tasks
async fn get_tasks_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { task_id }): Path<PageParams>,
) -> Result<Json<Value>> {
    debug!("{:<12} - get_task_handler", "HANLDER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    todo!()
}

async fn list_tasks_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Value>> {
    debug!("{:<12} - list_tasks_handler", "HANLDER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    // -- Get limit and offset from Query Params
    let limit = params.limit;
    let offset = params.offset;
    let order = params.order;

    todo!()
}

async fn create_tasks_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    // Json(payload): Json<UsersForCreatePayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - create_tasks_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    todo!()
}

async fn delete_tasks_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { task_id }): Path<PageParams>,
) -> Result<StatusCode> {
    debug!("{:<12} - delete_tasks_handler", "HANDLER");
    let ctx = ctxw.0;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    todo!()
}

async fn update_tasks_handler(
    State(mm): State<ModelManager>,
    ctxw: CtxW,
    Path(PageParams { task_id }): Path<PageParams>,
    // Json(payload): Json<UsersForUpdatePayload>,
) -> Result<(StatusCode, Json<Value>)> {
    debug!("{:<12} - update_user_handler", "HANDLER");
    let ctx = ctxw.0;
    let user_id_from_ctx = ctx.user_id().ok_or(Error::UserIdInCtxNotFound)?;

    // check authorize admin
    let is_authorized = UsersBmc::is_admin(&ctx, &mm).await?;
    if !is_authorized {
        return Err(Error::YourUserNotAuthorize);
    }

    todo!()
}

// endregion: --- Tasks
