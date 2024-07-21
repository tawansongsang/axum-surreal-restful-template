use axum::{middleware::from_fn_with_state, Router};
use lib_surrealdb::model::ModelManager;

use crate::middlewares::mw_ctx_resolve;

mod users;

pub fn route(mm: ModelManager) -> Router {
    let route = Router::new();
    route
        .merge(users::route(mm.clone()))
        .route_layer(from_fn_with_state(mm, mw_ctx_resolve))
}
