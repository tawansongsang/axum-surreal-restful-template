use std::sync::Arc;

use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, to_value};
use tracing::debug;

use super::Error;
use crate::{
    log::log_request,
    routes::{self, ClientError},
    // routes::{self, routes_rpc::RpcInfo},
};

use super::{auth::CtxW, req_stamp::ReqStamp};

pub async fn mw_response_map(
    ctx: Option<CtxW>,
    uri: Uri,
    req_method: Method,
    req_stamp: ReqStamp,
    res: Response,
) -> Response {
    let ctx = ctx.map(|ctx| ctx.0);

    debug!("{:<12} - mw_response_map", "RES_MAPPER");
    let uuid = req_stamp.uuid;
    // debug!("{:<12?} - res", res);

    // let rpc_info = res.extensions().get::<Arc<RpcInfo>>().map(Arc::as_ref);

    // -- Get the eventual response error.
    let middlewares_error = res.extensions().get::<Arc<Error>>().map(Arc::as_ref);
    let routes_error = res
        .extensions()
        // .get::<Arc<Error>>()
        .get::<Arc<routes::Error>>()
        .map(Arc::as_ref);
    let client_status_error =
        convert_middlewares_routes_error_to_client_error(middlewares_error, routes_error);
    // debug!("{:<12?} - client status error", client_status_error);

    // -- If client error, build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            debug!("{:<12} - client_status_error", "RES_MAPPER");
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let detail = client_error.as_ref().and_then(|v| v.get("detail"));

            let client_error_body = json!({
                // "id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
                "error": {
                    "message": message, // Variant name
                    "data": {
                        "req_uuid": uuid.to_string(),
                        "detail": detail
                    }
                }
            });

            debug!("CLIENT ERROR BODY:\n{client_error_body}");

            // -- Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- Build and log the server log line.
    let client_error = client_status_error.unzip().1;

    // TODO: Need to hander if log_request fail (but should not fail request)
    let _log_request = log_request(
        req_method,
        uri,
        req_stamp,
        // rpc_info,
        ctx,
        routes_error,
        middlewares_error,
        client_error,
    )
    .await;

    debug!("\n");

    error_response.unwrap_or(res)
}

fn convert_middlewares_routes_error_to_client_error(
    middlewares_error: Option<&Error>,
    routes_error: Option<&routes::Error>,
) -> Option<(StatusCode, ClientError)> {
    let client_status_error = middlewares_error.map(|se| se.client_status_and_error_middlewares());
    if client_status_error.is_some() {
        return client_status_error;
    }
    let client_status_error = routes_error.map(|se| se.client_status_and_error());
    client_status_error
}
