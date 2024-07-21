use axum::Router;
use lib_surrealdb::model::ModelManager;

mod v1;

pub fn route(mm: ModelManager) -> Router {
    let routes = Router::new();
    routes.nest("/api", v1::route(mm))
}
