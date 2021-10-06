use actix_web::{web, Scope};
use super::db;

#[derive(Clone)]
pub struct ApiState {
    pub db_conn_pool: db::DbConnPool,
}

impl ApiState {
    fn new() -> Self {
        ApiState {
            db_conn_pool: db::build_pool()
        }
    }
}

pub fn service() -> Scope {
    web::scope("/api")
        .data(ApiState::new())
}