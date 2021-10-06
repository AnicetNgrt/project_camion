use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use r2d2::Pool;

pub type DbConnPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_pool() -> DbConnPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let db_conn_manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(db_conn_manager)
        .expect("Could not build DB connection pool")
}