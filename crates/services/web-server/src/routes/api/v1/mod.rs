use axum::Router;
use lib_surrealdb::model::ModelManager;

mod login;
mod logout;
mod register;

pub fn all_routes(mm: ModelManager) -> Router {
    let all_routes = Router::new();
    all_routes
        .merge(login::route(mm.clone()))
        .merge(register::route(mm.clone()))
        .merge(logout::route(mm))
}

pub fn route(mm: ModelManager) -> Router {
    let route = Router::new();
    route.nest("/v1", all_routes(mm))
}
