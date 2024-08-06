pub mod bmc;

use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Debug, Deserialize)]
pub struct Tasks<'a> {
    pub id: sql::Thing,
    pub name: &'a str,
    pub owner: sql::Thing, // Users ID Table
    pub status: &'a str,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TasksGet<'a> {
    pub id: sql::Thing,
    pub name: &'a str,
    pub owner: sql::Thing, // Users ID Table
    pub status: &'a str,
    pub create_by: sql::Thing,
    pub create_on: sql::Datetime,
    pub update_by: sql::Thing,
    pub update_on: sql::Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TasksForUpdate {}

#[derive(Debug, Deserialize, Serialize)]
pub struct TasksForCreate {}
