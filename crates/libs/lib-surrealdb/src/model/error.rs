use super::store;
use derive_more::From;
use lib_auth::pwd;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },

    CannotValidateUsernameFromDB,
    CannotParseStrToDatetime(String),
    CannotParseStrToThing(String),
    CannotGetUserIdFromCtx,
    DataNotFound,
    DataNotFoundForCreated,
    DataNotFoundForDelete,
    DataNotFoundForUpdate,
    UserIdNotFound,
    UsernameAlreadyExists,
    UsernameNotValidFormat,

    // -- Modules
    #[from]
    Pwd(pwd::Error),
    #[from]
    Store(store::Error),

    // -- Externals
    #[from]
    Surrealdb(#[serde_as(as = "DisplayFromStr")] surrealdb::Error),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
