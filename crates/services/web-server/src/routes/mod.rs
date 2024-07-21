mod api;
mod error;
mod health;
pub mod static_file;

use axum::Router;
use lib_surrealdb::model::ModelManager;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

pub fn route(mm: ModelManager) -> Router {
    let routes = Router::new();
    routes.merge(health::route()).merge(api::route(mm))
}
