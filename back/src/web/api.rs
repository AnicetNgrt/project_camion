use crate::business::{db, users};
use actix_web::{get, http::StatusCode, post, web, Error, HttpResponse, Scope};
use serde_json::json;

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .app_data(web::Data::new(ApiState::new(db_conn_pool)))
        .service(ping)
        .service(register)
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

#[get("/ping")]
async fn ping() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("pong"))
}

#[post("/auth/register")]
async fn register(
    api_state: web::Data<ApiState>,
    register_data: web::Json<users::registration::Data>,
) -> HttpResponse {
    let (status, body) = match register_data.register(&&api_state.db_conn_pool).await {
        Ok(id) => (
            StatusCode::OK,
            json!({
                "registered": true,
                "id": id
            })
        ),
        Err(users::registration::Error::Data(issues)) => (
            StatusCode::OK,
            json!({
                "registered": false,
                "issues": issues
            })
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "error": error
            })
        ),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[post("/auth/login")]
async fn login(
    api_state: web::Data<ApiState>,
    login_data: web::Json<users::LoginData>,
) -> Result<HttpResponse, Error> {
    todo!("Implement login")
}
