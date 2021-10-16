use crate::core::{db};
use actix_web::{web, Scope};

mod auth;
mod users;
mod misc;
mod jokes;

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .app_data(web::Data::new(ApiState::new(db_conn_pool)))
        .service(misc::ping)
        .service(auth::register)
        .service(auth::login)
        .service(users::get_user_data)
        .service(users::search_users)
        .service(users::change_user_role)
        .service(jokes::create_joke)
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