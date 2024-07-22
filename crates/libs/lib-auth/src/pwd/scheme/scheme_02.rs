use std::sync::OnceLock;

use super::{Error, Result};
use crate::config::auth_config;
use crate::pwd::{scheme::Scheme, ContentToHash};
use argon2::password_hash::SaltString;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher as _, PasswordVerifier, Version,
};

pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash_ref = PasswordHash::new(pwd_ref).map_err(|_| Error::Hash)?;

        argon2
            .verify_password(to_hash.content.as_bytes(), &parsed_hash_ref)
            .map_err(|_| Error::PwdInValidate)
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().PWD_KEY;
        Argon2::new_with_secret(
            key,
            Algorithm::default(),
            Version::default(),
            Params::default(),
        )
        .unwrap() // TODO: needs to fail early
    })
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::pwd::ContentToHash;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_scheme_02_hash_into_b64u_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        let res = scheme.hash(&fx_to_hash)?;

        // -- Check
        assert_eq!(res, fx_res);

        Ok(())
    }

    #[test]
    fn test_scheme_02_hash_into_b64u_wrong_pass() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            // content: "hello world2".to_string(), // this is correct.
            content: "hello world2".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        // $argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$6bZW7re4W2G4rmxVI+9TttSmnU9zSO35K7UsEnMhe10
        let res = scheme.hash(&fx_to_hash)?;

        // -- Check
        assert_ne!(res, fx_res);

        Ok(())
    }

    #[test]
    fn test_scheme_02_hash_into_b64u_wrong_salt() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            // salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?, // this is correct uuid
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5451")?,
        };
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        // $argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUQ$PAJGAasj0uAw82Kl9PvuIAGC6CS0NAZrBNYMxF0eLQo
        let res = scheme.hash(&fx_to_hash)?;

        // -- Check
        assert_ne!(res, fx_res);

        Ok(())
    }

    #[test]
    fn test_scheme_02_validate_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_pass_ref = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        let res = scheme.validate(&fx_to_hash, &fx_pass_ref);

        // -- Check
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn test_scheme_02_validate_wrong_pass_ref() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_pass_ref = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        let res = scheme.validate(&fx_to_hash, &fx_pass_ref);

        // -- Check
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn test_scheme_02_validate_wrong_content() -> Result<()> {
        // -- Setup & Fixtures
        let fx_to_hash = ContentToHash {
            // content: "hello world".to_string(),
            content: "hello world2".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_pass_ref = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4";

        // -- Exec
        let scheme = Scheme02;
        let res = scheme.validate(&fx_to_hash, &fx_pass_ref);

        // -- Check
        assert!(res.is_err());

        Ok(())
    }
}
// endregion: --- Tests
