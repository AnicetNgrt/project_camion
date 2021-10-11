use crate::business::{db, security};
use super::{
    auth,
    email::string_is_email,
    queries::{
        find_by_email,
        find_by_username
    }
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Data {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub enum Error {
    Denied(DeniedReasons),
    Failure(Failure)
}

#[derive(Serialize)]
pub enum DeniedReasons {
    UnknownLogin, // Username wrong
    InvalidPassword, // Username good but password wrong
    InvalidCredentials // Email or password wrong (avoiding guessing attacks on emails)
}

#[derive(Serialize)]
pub enum Failure {
    Database,
    TokenCreation
}

impl Data {
    pub async fn login(&self, pool: &db::DbPool) -> Result<String, Error> {
        let login_is_email = string_is_email(&self.login);
        let maybe_user = if login_is_email {
            find_by_email(&self.login, pool).await
        } else {
            find_by_username(&self.login, pool).await
        };

        match (login_is_email, maybe_user) {
            (_, Ok(user)) => {
                if security::password_verify(&self.password, &user.password) {
                    auth::token_from_claims(user.id, user.role, 120)
                        .map_err(|_| Error::Failure(Failure::TokenCreation))
                } else {
                    if login_is_email {
                        Err(Error::Denied(DeniedReasons::InvalidCredentials))
                    } else {
                        Err(Error::Denied(DeniedReasons::InvalidPassword))
                    }
                }
            },
            (false, Err(sqlx::Error::RowNotFound)) => {
                Err(Error::Denied(DeniedReasons::UnknownLogin))
            },
            (true, Err(sqlx::Error::RowNotFound)) => {
                security::fake_password_verify(); // Avoiding guessing attacks on response time
                Err(Error::Denied(DeniedReasons::InvalidCredentials))
            },
            (_, Err(_)) => {
                Err(Error::Failure(Failure::Database))
            }
        }
    }
}