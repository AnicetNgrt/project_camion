use serde::{Serialize, Deserialize};

mod email;
mod username;
mod password;
mod queries;

pub use queries::{find_by_id};
pub mod auth;
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