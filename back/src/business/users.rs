use crate::business::{db, security};
use regex::Regex;
use serde::Deserialize;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
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

pub struct UserRegistrationResult {
    id: Option<i32>,
    username_issues: Option<Vec<UsernameIssues>>,
    email_issues: Option<Vec<EmailIssues>>,
    password_issues: Option<Vec<security::PasswordWeakness>>,
    hashing_error: Option<argon2::password_hash::Error>,
    database_error: Option<sqlx::Error>,
}

impl UserRegistrationData {
    pub async fn register(&self, pool: &db::DbPool) -> UserRegistrationResult {
        let mut result = UserRegistrationResult {
            id: None,
            username_issues: username_find_issues(&self.username, pool).await,
            email_issues: email_find_issues(&self.email, pool).await,
            password_issues: security::password_find_weaknesses(&self.password),
            hashing_error: None,
            database_error: None,
        };
        match security::password_salt_and_hash(&self.password) {
            Err(hashing_error) => result.hashing_error = Some(hashing_error),
            Ok(password_hash) => {
                if let UserRegistrationResult {
                    username_issues: None,
                    email_issues: None,
                    password_issues: None,
                    ..
                } = result
                {
                    match insert_user(&self.username, &self.email, &password_hash, pool).await {
                        Ok(id) => result.id = Some(id),
                        Err(db_error) => result.database_error = Some(db_error),
                    };
                }
            }
        }
        result
    }
}

async fn insert_user(username: &String, email: &String, password: &String, pool: &db::DbPool) -> Result<i32, sqlx::Error> {
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

pub enum UsernameIssues {
    CouldNotBeProcessed,
    TooShort,
    TooLong,
    NotUnique,
}

async fn username_find_issues(
    username: &String,
    pool: &db::DbPool,
) -> Option<Vec<UsernameIssues>> {
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
