use self::token::Claims;
use serde::{Deserialize, Serialize};

use super::db;

mod dl;
mod email;
mod password;
mod username;

pub mod login;
pub mod registration;
pub mod token;

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Author,
    None,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub role: UserRole,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub enum Error {
    NotFound,
    DataAccessLayerFailure,
}

impl User {
    pub fn is_searchable_by(&self, claims: &Option<Claims>) -> bool {
        let is_searcher = match claims {
            Some(Claims { id, .. }) => *id == self.id,
            None => false,
        };
        match claims {
            Some(Claims { role, .. }) => {
                self.role != UserRole::None || *role == UserRole::Admin || is_searcher
            }
            None => self.role != UserRole::None,
        }
    }

    pub fn to_json_as_seen_from(&self, claims: &Option<Claims>) -> serde_json::Value {
        if let Some(Claims { id, role }) = claims {
            if *id == self.id || *role == UserRole::Admin {
                return serde_json::json!({
                    "id": self.id,
                    "username": self.username,
                    "role": self.role,
                    "email": self.email
                });
            }
        }
        serde_json::json!({
            "id": self.id,
            "username": self.username,
            "role": self.role
        })
    }
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

pub async fn search_by_username_to_json_as_seen_from(
    query: &String,
    claims: &Option<Claims>,
    pool: &db::DbPool,
) -> Result<Vec<serde_json::Value>, Error> {
    let result = dl::search_by_username(query, pool).await;
    match result {
        Ok(users) => Ok(users
            .into_iter()
            .filter(|user| user.is_searchable_by(claims))
            .map(|user| user.to_json_as_seen_from(claims))
            .collect()),
        Err(_) => Err(Error::DataAccessLayerFailure),
    }
}

macro_rules! find_by_x {
    ($field:literal, $value:ident, $pool:ident) => {
        dl::find_by_x!($field, $value, $pool).map_err(|error| match error {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DataAccessLayerFailure,
        })
    };
}
use find_by_x;
