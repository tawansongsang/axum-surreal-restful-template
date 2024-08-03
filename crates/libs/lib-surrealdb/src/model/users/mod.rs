pub mod bmc;

use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Debug, Deserialize)]
pub struct Users {
    pub id: sql::Thing,
    pub username: String,
    pub email: String,
    pub email_verified: Option<sql::Datetime>,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,

    // -- pwd and token info
    pub password: String,
    pub password_salt: sql::Uuid,
    pub token_salt: sql::Uuid,

    pub image: Option<String>,
    pub role: String,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
    pub dateled_by: Option<sql::Thing>,
    pub deleted_on: Option<sql::Datetime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersGet {
    pub id: sql::Thing,
    pub username: String,
    pub email: String,
    pub email_verified: Option<sql::Datetime>,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub role: String,
    pub image: Option<String>,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Serialize)]
pub struct UsersCreated<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub email_verified: Option<sql::Datetime>,
    pub title: String,
    pub firstname: String,
    pub middlename: Option<String>,
    pub lastname: String,
    pub password: String,
    pub password_salt: sql::Uuid,
    pub create_by: &'a Option<sql::Thing>,
    pub update_by: &'a Option<sql::Thing>,
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
pub struct UsersForUpdate {
    // pub id: sql::Thing,
    pub username: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub lastname: Option<String>,
    pub image: Option<String>,
    pub role: Option<String>,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Serialize)]
pub struct UsersForDelete {
    pub deleted_by: sql::Thing,
    pub deleted_on: sql::Datetime,
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
    pub password_salt: sql::Uuid,
    pub token_salt: sql::Uuid,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct UsersForAuth {
    pub id: sql::Thing,
    pub username: String,
    pub role: String,

    // -- token info
    pub token_salt: sql::Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersRecord {
    pub id: sql::Thing,
}
