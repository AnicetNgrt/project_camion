use sqlx::postgres::{PgPoolOptions};
use sqlx::{Postgres, Pool};

pub type DbPool = Pool<Postgres>;

pub async fn build_pool(database_url: &String) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url).await.unwrap()
}

macro_rules! check_field_is_unique {
    ($table:literal, $field:literal, $value:ident, $pool:ident) => {
        match sqlx::query!(
            "SELECT " + $field + " FROM " + $table + " WHERE " + $field + " = $1",
            $value
        )
        .fetch_one($pool)
        .await
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
