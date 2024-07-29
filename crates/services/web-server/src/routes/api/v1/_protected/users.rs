use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use lib_surrealdb::model::{
    users::{bmc::UsersBmc, UsersGet},
    ModelManager,
};
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::{Error, Result};
use crate::{middlewares::auth::CtxW, params::PaginationParams};

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/users", get(api_get_users_handler))
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
