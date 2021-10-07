use actix_web::{web, get, post, Scope, Error, HttpResponse};
use crate::business::{db, users};

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .data(ApiState::new(db_conn_pool))
        .service(ping)
}

#[derive(Clone)]
pub struct ApiState {
    pub db_conn_pool: db::DbPool,
}

impl ApiState {
    fn new(db_conn_pool: db::DbPool) -> Self {
        ApiState {
            db_conn_pool
        }
    }
}

#[get("/ping")]
async fn ping() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("pong"))
}

#[post("/auth/register")]
async fn register(
    api_state: web::Data<ApiState>,
    signin_data: web::Json<users::UserRegistrationData>
) -> Result<HttpResponse, Error> {
    todo!("Implement register")
}

#[post("/auth/login")]
async fn login(
    api_state: web::Data<ApiState>,
    login_data: web::Json<users::UserLoginData>
) -> Result<HttpResponse, Error> {
    todo!("Implement login")
}