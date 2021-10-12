use serde::{Serialize, Deserialize};
use super::db;

mod email;
mod username;
mod password;
mod dl;

pub mod token;
pub mod registration;
pub mod login;

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name="user_role", rename_all="lowercase")]
pub enum UserRole {
    Admin,
    Author,
    None
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub role: UserRole,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub enum Error {
    NotFound,
    DataAccessLayerFailure
}

pub async fn find_by_id(id: i32, pool: &db::DbPool) -> Result<User, Error> {
    find_by_x!("id", id, pool)
}

pub async fn find_by_email(email: &String, pool: &db::DbPool) -> Result<User, Error> {
    find_by_x!("email", email, pool)
}

pub async fn find_by_username(username: &String, pool: &db::DbPool) -> Result<User, Error> {
    find_by_x!("username", username, pool)
}

macro_rules! find_by_x {
    ($field:literal, $value:ident, $pool:ident) => {
        dl::find_by_x!($field, $value, $pool)
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => Error::NotFound,
                _ => Error::DataAccessLayerFailure
            })
    };
}
use find_by_x;