use serde::Serialize;
use regex::Regex;
use crate::core::db;

#[derive(Serialize)]
pub enum Issues {
    CouldNotBeProcessed,
    Malformed,
    NotUnique,
}

pub fn string_is_email(email: &String) -> bool {
    let email_regex = Regex::new(
        r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
    )
    .unwrap();
    email_regex.is_match(email)
}

pub async fn find_issues(email: &String, pool: &db::DbPool) -> Option<Vec<Issues>> {
    let mut issues = vec![];

    if !string_is_email(email) {
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