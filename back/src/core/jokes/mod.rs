use super::{
    db,
    users::User
};
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

mod dl;

#[derive(Deserialize)]
pub struct JokeTemplate {
    pub title: String,
    pub lines: Vec<JokeLineTemplate>
}

#[derive(Serialize)]
pub enum Error {
    DataLayerFailure
}

impl JokeTemplate {
    pub async fn insert_and_set_author(&self, author: &User, pool: &db::DbPool) -> Result<Joke, Error> {
        dl::insert_joke(author, self, pool).await.map_err(|_| Error::DataLayerFailure)
    }
}

#[derive(Deserialize)]
pub struct JokeLineTemplate {
    pub speaker: String,
    pub content: String
}

#[derive(Serialize)]
pub struct Joke {
    pub id: i32,
    pub title: String,
    pub lines: Vec<JokeLine>,
    pub author_username: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime
}

#[derive(Serialize)]
pub struct JokeLine {
    pub id: i32,
    pub index_within_joke: i32,
    pub speaker: String,
    pub content: String,
}

