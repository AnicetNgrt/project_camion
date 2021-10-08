use crate::business::db;
use serde::Deserialize;
use regex::Regex;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub type UserRow = (i32, String, String, String);

impl From<UserRow> for User {
    fn from(row: (i32, String, String, String)) -> Self {
        User {
            id: row.0,
            username: row.1,
            email: row.2,
            password: row.3,
        }
    }
}

impl Into<UserRow> for User {
    fn into(self) -> UserRow {
        (self.id, self.username, self.email, self.password)
    }
}

#[derive(Deserialize)]
pub struct UserRegistrationData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLoginData {
    pub login: String,
    pub password: String,
}

pub enum UserRegistrationError {
    UsernameTaken,
    UsernameInvalid(UsernameInvalidReason),
    EmailTaken,
    EmailInvalid(EmailInvalidReason),
    PasswordWeak,
    DatabaseError(sqlx::Error),
}

impl UserRegistrationData {
    pub async fn register(&self, pool: &db::DbPool) -> Result<i32, UserRegistrationError> {
        match username_check_validity(&self.username) {
            Err(reason) => return Err(UserRegistrationError::UsernameInvalid(reason)),
            _ => ()
        };
        
        match email_check_validity(&self.email) {
            Err(reason) => return Err(UserRegistrationError::EmailInvalid(reason)),
            _ => ()
        };

        match username_is_unique(&self.username, pool).await {
            Ok(false) => return Err(UserRegistrationError::UsernameTaken),
            Err(db_error) => return Err(UserRegistrationError::DatabaseError(db_error)),
            _ => ()
        };
        
        match email_is_unique(&self.email, pool).await {
            Ok(false) => return Err(UserRegistrationError::EmailTaken),
            Err(db_error) => return Err(UserRegistrationError::DatabaseError(db_error)),
            _ => ()
        };

        match sqlx::query!(
            r#"
            INSERT INTO users ( username, email, password )
            VALUES ( $1, $2, $3 )
            RETURNING id
            "#,
            self.username,
            self.email,
            self.password
        )
        .fetch_one(pool)
        .await
        {
            Ok(record) => Ok(record.id),
            Err(db_error) => Err(UserRegistrationError::DatabaseError(db_error)),
        }
    }
}

pub enum UsernameInvalidReason {
    TooShort,
    TooLong
}

pub fn username_check_validity(username: &String) -> Result<(), UsernameInvalidReason> {
    if username.len() < 3 {
        return Err(UsernameInvalidReason::TooShort);
    }
    if username.len() > 32 {
        return Err(UsernameInvalidReason::TooLong);
    }
    Ok(())
}

pub async fn username_is_unique(username: &String, pool: &db::DbPool) -> Result<bool, sqlx::Error> {
    db::check_field_is_unique!("users", "username", username, pool)
}

pub enum EmailInvalidReason {
    Malformed
}

pub fn email_check_validity(email: &String) -> Result<(), EmailInvalidReason> {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(EmailInvalidReason::Malformed)
    }
}

pub async fn email_is_unique(email: &String, pool: &db::DbPool) -> Result<bool, sqlx::Error> {
    db::check_field_is_unique!("users", "email", email, pool)
}