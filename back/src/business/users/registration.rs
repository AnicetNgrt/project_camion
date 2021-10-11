use crate::business::{db};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize)]
pub struct Data {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub enum Error {
    Data(DataIssues),
    Failure(Failures)
}

#[derive(Serialize)]
pub enum Failures {
    PasswordHashing,
    DatabaseInsertion,
}

type DataIssues = (
    Option<Vec<username::Issues>>,
    Option<Vec<email::Issues>>,
    Option<Vec<password::Weakness>>,
);

impl Data {
    pub async fn register(&self, pool: &db::DbPool) -> Result<i32, Error> {
        if let Some(issues) = self.find_issues(&pool).await {
            Err(Error::Data(issues))
        } else if let Ok(hashed_password) = password::hash(&self.password) {
            queries::insert_returning_id(
                &self.username,
                &self.email,
                &hashed_password,
                UserRole::None,
                pool,
            )
            .await
            .map_err(|_| Error::Failure(Failures::DatabaseInsertion))
        } else {
            Err(Error::Failure(Failures::PasswordHashing))
        }
    }

    async fn find_issues(&self, pool: &db::DbPool) -> Option<DataIssues> {
        match (
            username::find_issues(&self.username, pool).await,
            email::find_issues(&self.email, pool).await,
            password::find_weaknesses(&self.password),
        ) {
            (None, None, None) => None,
            issues => Some(issues),
        }
    }
}