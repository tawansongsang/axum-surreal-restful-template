use async_trait::async_trait;
use lib_auth::token;
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        users::{bmc::UsersBmc, Users},
        ModelManager,
    },
};
use serde::Serialize;

use super::error::{Error, Result};
use axum::{
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, HeaderMap},
    middleware::Next,
    response::Response,
};
use tracing::debug;

pub async fn mw_ctx_resolve(
    mm: State<ModelManager>,
    headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve {:?}", "MIDDLEWARE", headers);
    let mut authorization = headers
        .get("Authorization")
        .ok_or(Error::NoAuthorizationHeader)?
        .to_str()
        .map_err(|_| Error::CannotConvertAuthorizationToStr)?
        .split_whitespace();
    let _bearer = authorization
        .next()
        .ok_or(Error::NoAuthorizationBearer)
        .map(|b| {
            if b.eq("Bearer") {
                return Ok(());
            } else {
                return Err(Error::NoAuthorizationBearer);
            }
        })??;
    let token = authorization.next().ok_or(Error::InvalidBearerToken)?;

    debug!("{:<12} - mw_ctx_resolve {:?}", "MIDDLEWARE", token);

    let ctx_ext_result = inner_ctx_resolve(mm, token).await;

    // -- Store the ctx_ext_result in the request extension
    // (for Ctx extractor)
    let _ctxw = req.extensions_mut().insert(ctx_ext_result);

    let response = next.run(req).await;

    Ok(response)
}

async fn inner_ctx_resolve(mm: State<ModelManager>, token: &str) -> CtxExtResult {
    let user_id = token::decode_kid_from_jwt_headers(token)
        .map_err(|_| CtxExtError::InvalidJwtTokenHeader)?;
    let _ctx = Ctx::root_ctx();
    let user = UsersBmc::get::<Users>(&_ctx, &mm, &user_id.as_str())
        .await
        .map_err(|e| CtxExtError::ModelAccessError(e.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    let token_salt = user.token_salt.as_ref();
    let sub = token::decode_sub_from_jwt(token, token_salt)
        .map_err(|e| CtxExtError::JwtDecodeError(e.to_string()))?;
    let ctxw = Ctx::new(Some(sub))
        .map_err(|_| CtxExtError::CannotCreateCtxFromJwt)
        .map(CtxW);
    ctxw
}

// region:    --- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        let part = parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt);

        part
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = std::result::Result<CtxW, CtxExtError>;

#[derive(Debug, Serialize, Clone)]
pub enum CtxExtError {
    JwtDecodeError(String),
    InvalidJwtTokenHeader,
    CannotCreateCtxFromJwt,

    ModelAccessError(String),
    UserNotFound,
    CtxNotInRequestExt,
}
// endregion: --- Ctx Extractor Result/Error
