use axum::Router;
use lib_surrealdb::model::ModelManager;

mod _protected;
mod login;
mod logout;

pub fn routes_all(mm: ModelManager) -> Router {
    let routes_all = Router::new();
    routes_all
        .merge(login::route(mm.clone()))
        .merge(logout::route(mm.clone()))
        .merge(_protected::route(mm))
}

pub fn route(mm: ModelManager) -> Router {
    let route = Router::new();
    route.nest("/v1", routes_all(mm))
}
