pub mod bmc;

use serde::{Deserialize, Serialize};
use surrealdb::sql;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Users {
    pub id: sql::Thing,
    pub username: String,
    pub email: String,
    pub email_verified: sql::Datetime,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,

    // -- pwd and token info
    pub password: String,
    pub password_salt: Uuid,
    pub token_salt: Uuid,

    pub image: String,
    pub role: String,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Deserialize)]
pub struct UsersGet {
    pub id: sql::Thing,
    pub username: String,
    pub email: String,
    pub email_verified: sql::Datetime,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub role: String,
    pub image: String,

    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Serialize)]
pub struct UsersForCreate {
    pub username: String,
    pub email: String,
    // pub email_verified: sql::Datetime,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UsersCreated<'a> {
    pub username: &'a str,
    pub email: &'a str,
    // pub email_verified: sql::Datetime,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub password: String,
    pub create_by: &'a Option<sql::Thing>,
    pub update_by: &'a Option<sql::Thing>,
}

#[derive(Debug, Deserialize)]
pub struct UsersForLogin {
    pub id: sql::Thing,
    pub username: String,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub password: Option<String>, // encrypted, #_scheme_id_#....
    pub password_salt: Uuid,
    pub token_salt: Uuid,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UsersForAuth {
    pub id: sql::Thing,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UsersRecord {
    pub id: sql::Thing,
}
