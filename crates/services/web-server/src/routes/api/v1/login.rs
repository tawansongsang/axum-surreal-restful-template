use axum::{extract::State, routing::post, Json, Router};
use lib_auth::{pwd, token};
use lib_surrealdb::{
    ctx::Ctx,
    model::{
        users::{bmc::UsersBmc, UsersForLogin},
        ModelManager,
    },
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::routes::{Error, Result};

pub fn route(mm: ModelManager) -> Router {
    Router::new()
        .route("/login", post(api_login_handler))
        .with_state(mm)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login_handler(
    State(mm): State<ModelManager>,
    // cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANLDER");

    let LoginPayload { username, password } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user.
    let user: UsersForLogin = UsersBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;

    let user_id = user.id.id.to_raw();
    let token_salt = user.token_salt;

    // -- Validate the password.
    let Some(hash) = user.password else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    let to_hash = pwd::ContentToHash::new(password, uuid::Uuid::from(user.password_salt));

    let _scheme_status =
        pwd::validate_pwd(to_hash, hash)
            .await
            .map_err(|_| Error::LoginFailPwdNotMatching {
                user_id: user_id.clone(),
            })?;

    // -- Update password scheme if needed
    // if let SchemeStatus::Outdated = scheme_status {
    //     debug!("pwd encrypt scheme outdated, upgrading.");
    //     UserBmc::update_pwd(&root_ctx, &mm, user.id, &pwd_clear).await?;
    // }

    // -- Set web token if not send back token via body
    // web::set_token_cookie(&cookies, &user_id, user.token_salt)?;

    // -- Generate toekn if not use cookie
    let jwt = token::encode_jwt(user_id.as_str(), token_salt.as_ref())?;

    // -- Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        },
        "data": {
            "id": user_id,
            "email": user.username,
            "title": user.title,
            "firstname": user.firstname,
            "middlename": user.middlename,
            "lastname": user.lastname,
            "role": user.role,
            "image": null,
        },
        "jwt": jwt
    }));

    Ok(body)
}
