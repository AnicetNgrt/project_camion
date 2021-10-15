use super::{
    db,
    users::User
};
use chrono::{NaiveDateTime};

mod dl;

pub struct JokeTemplate {
    pub title: String,
    pub lines: Vec<JokeLineTemplate>
}

pub enum Error {
    DataLayerFailure
}

impl JokeTemplate {
    pub async fn insert_and_set_author(&self, author: &User, pool: &db::DbPool) -> Result<Joke, Error> {
        dl::insert_joke(author, self, pool).await.map_err(|_| Error::DataLayerFailure)
    }
}

pub struct JokeLineTemplate {
    pub speaker: String,
    pub content: String
}

pub struct Joke {
    pub id: i32,
    pub title: String,
    pub lines: Vec<JokeLine>,
    pub author: User,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime
}

pub struct JokeLine {
    pub id: i32,
    pub index_within_joke: i32,
    pub speaker: String,
    pub content: String,
}

