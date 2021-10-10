use crate::business::{db, security};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegistrationData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub enum RegistrationError {
    Data(RegistrationDataIssues),
    PasswordHashing,
    DatabaseInsertion,
}

type RegistrationDataIssues = (
    Option<Vec<UsernameIssues>>,
    Option<Vec<EmailIssues>>,
    Option<Vec<security::PasswordWeakness>>,
);

impl RegistrationData {
    async fn find_issues(&self, pool: &db::DbPool) -> Option<RegistrationDataIssues> {
        match (
            username_find_issues(&self.username, pool).await,
            email_find_issues(&self.email, pool).await,
            security::password_find_weaknesses(&self.password),
        ) {
            (None, None, None) => None,
            issues => Some(issues),
        }
    }

    fn hash_password(&self) -> Result<String, ()> {
        security::password_salt_and_hash(&self.password)
            .map_err(|_| ())
    }

    pub async fn register(&self, pool: &db::DbPool) -> Result<i32, RegistrationError> {
        if let Some(issues) = self.find_issues(&pool).await {
            Err(RegistrationError::Data(issues))
        } else if let Ok(hashed_password) = self.hash_password() {
            insert_user(
                &self.username,
                &self.email,
                &hashed_password,
                pool,
            )
            .await
            .map_err(|_| RegistrationError::DatabaseInsertion)
        } else {
            Err(RegistrationError::PasswordHashing)
        }
    }
}

async fn insert_user(
    username: &String,
    email: &String,
    password: &String,
    pool: &db::DbPool,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
    INSERT INTO users ( username, email, password )
    VALUES ( $1, $2, $3 )
    RETURNING id
    "#,
        username,
        email,
        password
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

#[derive(Serialize)]
pub enum UsernameIssues {
    CouldNotBeProcessed,
    TooShort,
    TooLong,
    NotUnique,
}

async fn username_find_issues(username: &String, pool: &db::DbPool) -> Option<Vec<UsernameIssues>> {
    let mut issues = vec![];

    if username.len() < 3 {
        issues.push(UsernameIssues::TooShort);
    } else if username.len() > 32 {
        issues.push(UsernameIssues::TooLong);
    } else {
        match username_is_unique(username, pool).await {
            Ok(false) => issues.push(UsernameIssues::NotUnique),
            Err(_) => issues.push(UsernameIssues::CouldNotBeProcessed),
            _ => (),
        };
    }

    if !issues.is_empty() {
        Some(issues)
    } else {
        None
    }
}

async fn username_is_unique(username: &String, pool: &db::DbPool) -> Result<bool, sqlx::Error> {
    db::check_field_is_unique!("users", "username", username, pool)
}

#[derive(Serialize)]
pub enum EmailIssues {
    CouldNotBeProcessed,
    Malformed,
    NotUnique,
}

async fn email_find_issues(email: &String, pool: &db::DbPool) -> Option<Vec<EmailIssues>> {
    let mut issues = vec![];

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(email) {
        issues.push(EmailIssues::Malformed);
    } else {
        match email_is_unique(email, pool).await {
            Ok(false) => issues.push(EmailIssues::NotUnique),
            Err(_) => issues.push(EmailIssues::CouldNotBeProcessed),
            _ => (),
        };
    }

    if !issues.is_empty() {
        Some(issues)
    } else {
        None
    }
}

async fn email_is_unique(email: &String, pool: &db::DbPool) -> Result<bool, sqlx::Error> {
    db::check_field_is_unique!("users", "email", email, pool)
}

#[derive(Serialize)]
struct UserAuthJwtClaims {
    id: i32,
    exp: usize,
}

pub fn auth_jwt_from_id(id: i32, expiration_sec: i64) -> Result<String, ()> {
    security::jwt_create(&mut UserAuthJwtClaims { id, exp: 0 }, expiration_sec)
}

impl security::JwtClaims for UserAuthJwtClaims {
    fn set_expiration(&mut self, exp: usize) {
        self.exp = exp;
    }
}
