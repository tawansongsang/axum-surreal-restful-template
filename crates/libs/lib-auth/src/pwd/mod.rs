mod error;
mod scheme;

use std::str::FromStr;

use lazy_regex::regex_captures;
use uuid::Uuid;

pub use self::error::{Error, Result};
pub use self::scheme::{get_scheme, Scheme, SchemeStatus, DEFAULT_SCHEME};

// region:    --- Types
/// The clean content to hash, with the salt.
///
/// Notes:
///    - Since content is sensitive information, we do NOT implement default debug for this struct.
///    - The clone is only implement for testing
#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
    pub content: String, // Clear content.
    pub salt: Uuid,
}

impl ContentToHash {
    pub fn new(content: String, salt: Uuid) -> Self {
        ContentToHash { content, salt }
    }
}
// endregion: --- Types

/// Hash the password with the default scheme.
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
    tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
        .await
        .map_err(|_| Error::FailSpawnBlockForHash)?
}

/// Validate if an ContentToHash matches.
pub async fn validate_pwd(to_hash: ContentToHash, pwd_ref: String) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse()?;

    // Note: We do first, so that we do not have to  clone the scheme_name.
    let scheme_status = if scheme_name == DEFAULT_SCHEME {
        SchemeStatus::Ok
    } else {
        SchemeStatus::Outdated
    };

    // Note: Since validate might take some time depending on algo
    //       doing a spawn_blocking to avoid
    tokio::task::spawn_blocking(move || validate_for_scheme(&scheme_name, to_hash, hashed))
        .await
        .map_err(|_| Error::FailSpawnBlockForValidate)??;

    // validate_for_scheme(&scheme_name, to_hash, hashed)?;
    Ok(scheme_status)
}

fn validate_for_scheme(scheme_name: &str, to_hash: ContentToHash, pwd_ref: String) -> Result<()> {
    let _ = get_scheme(scheme_name)?.validate(&to_hash, &pwd_ref)?;
    Ok(())
}

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
    let pwd_hashed = get_scheme(scheme_name)?.hash(&to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

struct PwdParts {
    /// The scheme only (e.g., "01")
    scheme_name: String,
    /// The hashed password,
    hashed: String,
}

impl FromStr for PwdParts {
    type Err = Error;

    fn from_str(pwd_with_scheme: &str) -> Result<Self> {
        regex_captures!(
            r#"^#(\w+)#(.*)"#, // a literal regex
            pwd_with_scheme
        )
        .map(|(_, scheme, hashed)| Self {
            scheme_name: scheme.to_string(),
            hashed: hashed.to_string(),
        })
        .ok_or(Error::PwdWithSchemeFailedParse)
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_multi_scheme_outdated() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: fx_salt,
        };

        // -- Exec
        let pwd_hashed = hash_for_scheme("01", fx_to_hash.clone())?;
        let pwd_validate = validate_pwd(fx_to_hash.clone(), pwd_hashed).await?;

        // -- Check
        assert!(
            matches!(pwd_validate, SchemeStatus::Outdated),
            "status should be SchemeStatus::Outdated"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_scheme_valid_content_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: fx_salt,
        };
        let fx_pwd_ref = "#02#$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4".to_string();

        // -- Exec
        let pwd_validate = validate_pwd(fx_to_hash.clone(), fx_pwd_ref).await?;

        // -- Check
        assert!(matches!(pwd_validate, SchemeStatus::Ok));

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_scheme_invalid_content() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
        let fx_to_hash = ContentToHash {
            content: "hello world2".to_string(), // hello world
            salt: fx_salt,
        };
        let fx_pwd_ref = "#02#$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$TaRnmmbDdQ1aTzk2qQ2yQzPQoZfnKqhrfuTH/TRP5V4".to_string();

        // -- Exec
        let _pwd_hashed = hash_for_scheme("02", fx_to_hash.clone())?;
        let pwd_validate = validate_pwd(fx_to_hash.clone(), fx_pwd_ref).await.is_err();

        // -- Check
        assert!(pwd_validate);

        Ok(())
    }
}
// endregion: --- Tests
