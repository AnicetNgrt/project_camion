use serde::Serialize;
use regex::Regex;
use crate::business::db;

#[derive(Serialize)]
pub enum Issues {
    CouldNotBeProcessed,
    Malformed,
    NotUnique,
}

pub async fn find_issues(email: &String, pool: &db::DbPool) -> Option<Vec<Issues>> {
    let mut issues = vec![];

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if !email_regex.is_match(email) {
        issues.push(Issues::Malformed);
    } else {
        match email_is_unique(email, pool).await {
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

async fn email_is_unique(email: &String, pool: &db::DbPool) -> Result<bool, sqlx::Error> {
    db::check_field_is_unique!("users", "email", email, pool)
}