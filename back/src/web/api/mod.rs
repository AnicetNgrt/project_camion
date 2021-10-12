use crate::core::{db};
use actix_web::{web, Scope};

mod auth;
mod users;
mod misc;

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .app_data(web::Data::new(ApiState::new(db_conn_pool)))
        .service(misc::ping)
        .service(auth::register)
        .service(auth::login)
        .service(users::user_detail)
}

#[derive(Clone)]
pub struct ApiState {
    pub db_conn_pool: db::DbPool,
}

impl ApiState {
    fn new(db_conn_pool: db::DbPool) -> Self {
        ApiState { db_conn_pool }
    }
}