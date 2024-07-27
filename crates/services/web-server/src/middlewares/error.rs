use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};
use derive_more::From;
use serde::Serialize;
use serde_with::serde_as;
use tracing::debug;

use crate::routes::ClientError;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- ReqStamp
    ReqStampNotInResponseExt,

    // -- Headers
    NoAuthorizationBearer,
    NoAuthorizationHeader,
    CannotConvertAuthorizationToStr,
    InvalidBearerToken,

    // -- CtxExtError
    #[from]
    CtxExt(super::auth::CtxExtError),
    // -- Modules
    // Token(token::Error),
    // Model(model::Error),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        debug!("{:<12} - middlewares::Error {self:?}", "INTO_RES");

        // -- Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // -- Insert the Error into the response.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl Error {
    pub fn client_status_and_error_middlewares(&self) -> (StatusCode, ClientError) {
        use super::Error::*;
        match self {
            // -- Headers
            NoAuthorizationBearer
            | NoAuthorizationHeader
            | CannotConvertAuthorizationToStr
            | InvalidBearerToken => (
                StatusCode::FORBIDDEN,
                ClientError::INVALID_AUTHORIZATION_HEADER,
            ),

            // -- Auth
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // -- Fallback,
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
