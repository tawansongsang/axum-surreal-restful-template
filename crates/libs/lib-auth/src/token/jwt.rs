use super::{error::Result, Error};
use jsonwebtoken::{
    decode, decode_header, encode, get_current_timestamp, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

use crate::auth_config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn encode_jwt(sub: &str, secret_key: &[u8]) -> Result<String> {
    let jwt_duration_sec = auth_config().TOKEN_DURATION_SEC_USIZE;
    let current_time = get_current_timestamp() as usize;
    let exp = current_time + jwt_duration_sec;
    let claim = Claims {
        sub: sub.to_string(),
        exp,
    };
    let mut headers = Header::default();
    headers.kid = Some(sub.to_string());

    let jwt = encode(&headers, &claim, &EncodingKey::from_secret(secret_key))?;

    Ok(jwt)
}

pub fn decode_kid_from_jwt_headers(jwt: &str) -> Result<String> {
    decode_header(jwt)?.kid.ok_or(Error::JwtNoKid)
}

pub fn decode_sub_from_jwt(ori_jwt: &str, secret_key: &[u8]) -> Result<String> {
    let sub = decode::<Claims>(
        ori_jwt,
        &DecodingKey::from_secret(secret_key),
        &Validation::default(),
    )?
    .claims
    .sub;

    Ok(sub)
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Error;
    use anyhow::Result;

    #[test]
    fn test_gen_jwt_ok() -> Result<()> {
        let sub = "iR1f8i7Wg7jipR3uhDhJ";
        let token_salt = "36ee060e-20a7-4a42-8bd1-0cbd704a29a2";
        let _jwt = self::encode_jwt(sub, token_salt.as_ref()).unwrap();

        Ok(())
    }

    #[test]
    fn test_get_kid_header_jwt() -> Result<()> {
        // -- Setup & Fixtures
        let fx_kid = "iR1f8i7Wg7jipR3uhDhJ";
        let sub = "iR1f8i7Wg7jipR3uhDhJ";
        let token_salt = "36ee060e-20a7-4a42-8bd1-0cbd704a29a2";
        let jwt = self::encode_jwt(sub, token_salt.as_ref()).unwrap();

        let kid = decode_kid_from_jwt_headers(jwt.as_str()).unwrap();

        // -- Check
        assert_eq!(kid, fx_kid);

        Ok(())
    }

    #[test]
    fn test_validate_and_get_sub_jwt() -> Result<()> {
        // -- Setup & Fixtures
        let fx_sub = "iR1f8i7Wg7jipR3uhDhJ";
        let sub = "iR1f8i7Wg7jipR3uhDhJ";
        let token_salt = "36ee060e-20a7-4a42-8bd1-0cbd704a29a2";
        let ori_jwt = self::encode_jwt(sub, token_salt.as_ref()).unwrap();

        let sub = decode_sub_from_jwt(&ori_jwt, token_salt.as_ref()).unwrap();

        // -- Check
        assert_eq!(sub, fx_sub);

        Ok(())
    }

    #[test]
    fn test_expired_get_sub_jwt_error() -> Result<()> {
        // -- Setup & Fixtures
        let fx_jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiIsImtpZCI6ImlSMWY4aTdXZzdqaXBSM3VoRGhKIn0.eyJzdWIiOiJpUjFmOGk3V2c3amlwUjN1aERoSiIsImV4cCI6MTcyMTYzNjM4OH0.pxZ2bXsW9frjXaK9OsmHvVy_D66HMDlTAIG1HdPfIX4";
        let token_salt = "36ee060e-20a7-4a42-8bd1-0cbd704a29a2";

        let sub = decode_sub_from_jwt(fx_jwt, token_salt.as_ref());
        let error =
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature);
        let expected = Err(Error::JsonWebToken(error));

        // -- Check
        assert_eq!(sub, expected);

        Ok(())
    }
}
// endregion: --- Tests
