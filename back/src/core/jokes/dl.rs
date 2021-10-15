use super::super::users::{self, User};
use super::{Joke, JokeLine, JokeTemplate};
use crate::core::db;
use chrono::{NaiveDateTime, Utc};

pub struct JokePostgres {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

impl JokePostgres {
    pub async fn to_joke(self, lines_pg: Vec<JokeLinePostgres>, pool: &db::DbPool) -> Joke {
        let lines: Vec<JokeLine> = lines_pg.into_iter().map(|j| j.into()).collect();
        let author = users::find_by_id(self.author_id, pool).await.expect(&format!("Tried converting joke_pg to joke but author with id {} does not exist. Incoherent data.", self.author_id));
        Joke {
            id: self.id,
            title: self.title,
            lines,
            author,
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
}

pub struct JokeLinePostgres {
    pub id: i32,
    pub index_within_joke: i32,
    pub joke_id: i32,
    pub speaker: String,
    pub content: String,
}

impl Into<JokeLine> for JokeLinePostgres {
    fn into(self) -> JokeLine {
        JokeLine {
            id: self.id,
            index_within_joke: self.index_within_joke,
            speaker: self.speaker,
            content: self.content,
        }
    }
}

pub async fn insert_joke(
    author: &User,
    template: &JokeTemplate,
    pool: &db::DbPool,
) -> Result<Joke, sqlx::Error> {
    let now = Utc::now().naive_utc();

    let joke_pg = sqlx::query_as!(
        JokePostgres,
        r#"
        INSERT INTO jokes ( title, author_id, created_at, modified_at )
        VALUES ( $1, $2, $3, $4 )
        RETURNING *
        "#,
        template.title,
        author.id,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    let mut lines_pg = Vec::<JokeLinePostgres>::new();
    let mut i = 0;
    for line_template in template.lines.iter() {
        let maybe_line_pg = sqlx::query_as!(
            JokeLinePostgres,
            r#"
            INSERT INTO joke_lines ( speaker, content, index_within_joke, joke_id )
            VALUES ( $1, $2, $3, $4 )
            RETURNING *
            "#,
            line_template.speaker,
            line_template.content,
            i,
            joke_pg.id
        )
        .fetch_one(pool)
        .await;

        match maybe_line_pg {
            Ok(line_pg) => lines_pg.push(line_pg),
            Err(error) => {
                sqlx::query!("DELETE FROM jokes WHERE id = $1", joke_pg.id)
                    .execute(pool)
                    .await
                    .expect(&format!(
                        "Tried inserting joke with id {}. One of the line couldn't be inserted. Tried deleting parent joke but it failed.",
                        joke_pg.id
                    ));
                return Err(error);
            }
        };

        i += 1;
    }

    Ok(joke_pg.to_joke(lines_pg, pool).await)
}
