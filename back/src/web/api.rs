use crate::business::{db, users};
use actix_web::{get, post, web, Error, HttpResponse, Scope};

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .data(ApiState::new(db_conn_pool))
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
    register_data: web::Json<users::RegistrationData>,
) -> HttpResponse {
    let mut register_data = register_data;
    match register_data.register(&&api_state.db_conn_pool).await {
        Err(issues) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!(
                "{{ success: false, issues: {} }}",
                serde_json::to_string(&issues).unwrap()
            )),
        Ok(jwt) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{ success: true, jwt: {} }}", jwt)),
    }
}

#[post("/auth/login")]
async fn login(
    api_state: web::Data<ApiState>,
    login_data: web::Json<users::LoginData>,
) -> Result<HttpResponse, Error> {
    todo!("Implement login")
}
