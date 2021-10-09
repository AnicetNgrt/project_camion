use std::env;
use sqlx::postgres::{PgPoolOptions};
use sqlx::{Postgres, Pool};

pub type DbPool = Pool<Postgres>;

pub async fn build_pool() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap()
}

macro_rules! query_one_where_field_equals {
    ($table:literal, $field:literal, $value:ident, $pool:ident) => {
        sqlx::query!(
            "SELECT * FROM " + $table + " WHERE " + $field + " = $1",
            $value
        )
        .fetch_one($pool)
        .await
    };
}
pub(crate) use query_one_where_field_equals;

macro_rules! check_field_is_unique {
    ($table:literal, $field:literal, $value:ident, $pool:ident) => {
        match db::query_one_where_field_equals!($table, $field, $value, $pool)
        {
            Ok(_) => Ok(false),
            Err(db_error) => match db_error {
                sqlx::Error::RowNotFound => Ok(true),
                _ => Err(db_error)
            }
        }
    };
}
pub(crate) use check_field_is_unique;
