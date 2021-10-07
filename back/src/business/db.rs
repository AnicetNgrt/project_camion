use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres, Pool};

pub type DbPool = Pool<Postgres>;

pub async fn build_pool() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap()
}