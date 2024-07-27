mod config;
mod error;
mod log;
mod middlewares;
mod routes;

use axum::{middleware, Router};
use lib_surrealdb::model::ModelManager;
use tokio::net::TcpListener;
// use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub use error::Result;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting Connection to SurrealDB");

    // -- Initialize ModelManager.
    let mm = ModelManager::new().await.unwrap();

    // -- Testing Connection to DB
    mm.test_connection().await;

    info!("Starting Service ...");

    // let rpc_state = RpcState { mm: mm.clone() };
    // let routes_rpc =
    //     web::routes_rpc::routes(rpc_state).route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes::route(mm.clone()))
        // .merge(routes_login::routes(mm.clone()))
        // .nest("/api", routes_rpc)
        .layer(middleware::map_response(middlewares::mw_response_map))
        .layer(middleware::from_fn(middlewares::mw_req_stamp))
        // .layer(CookieManagerLayer::new())
        .fallback_service(routes::static_file::serve_dir());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}
