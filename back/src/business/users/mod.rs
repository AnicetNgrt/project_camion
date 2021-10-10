use serde::{Deserialize};

mod email;
mod username;
mod password;
mod jwt;
mod queries;

pub mod registration;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub login: String,
    pub password: String,
}