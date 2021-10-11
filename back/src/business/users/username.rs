use serde::Serialize;
use crate::business::db;
use super::email::string_is_email;

#[derive(Serialize)]
pub enum Issues {
    CouldNotBeProcessed,
    // If we allow users to put emails as usernames it makes the login
    // process more hazardous
    EmailLike,
    TooShort,
    TooLong,
    NotUnique,
}

pub async fn find_issues(username: &String, pool: &db::DbPool) -> Option<Vec<Issues>> {
    let mut issues = vec![];

    if string_is_email(username) {
        issues.push(Issues::EmailLike);
    }

    if username.len() < 3 {
        issues.push(Issues::TooShort);
    } else if username.len() > 32 {
        issues.push(Issues::TooLong);
    } else {
        match username_is_unique(username, pool).await {
            Ok(false) => issues.push(Issues::NotUnique),
            Err(_) => issues.push(Issues::CouldNotBeProcessed),
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