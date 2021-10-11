use crate::business::{db, users};
use actix_web::{get, http::StatusCode, post, web, Error, HttpRequest, HttpResponse, Scope};
use serde::Serialize;
use serde_json::json;

pub fn service(db_conn_pool: db::DbPool) -> Scope {
    web::scope("/api")
        .app_data(web::Data::new(ApiState::new(db_conn_pool)))
        .service(ping)
        .service(register)
        .service(login)
        .service(user_detail)
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
            }),
        ),
        Err(users::registration::Error::Data(issues)) => (
            StatusCode::OK,
            json!({
                "registered": false,
                "issues": issues
            }),
        ),
        Err(users::registration::Error::Failure(failure)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": failure }),
        ),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[post("/auth/login")]
async fn login(
    api_state: web::Data<ApiState>,
    login_data: web::Json<users::login::Data>,
) -> HttpResponse {
    let (status, body) = match login_data.login(&&api_state.db_conn_pool).await {
        Ok(jwt) => (StatusCode::OK, json!({ "jwt": jwt })),
        Err(users::login::Error::Denied(reason)) => {
            (StatusCode::UNAUTHORIZED, json!({ "reason": reason }))
        }
        Err(users::login::Error::Failure(failure)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": failure }),
        ),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[get("/user/{id}")]
async fn user_detail(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    enforce_id!(&req, path.0);
    let (status, body) = match users::find_by_id(path.0, &&api_state.db_conn_pool).await {
        Ok(user) => (
            StatusCode::OK,
            json!({
                "email": user.email
            }),
        ),
        Err(users::Error::NotFound) => (StatusCode::NOT_FOUND, json!({})),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({
                "error": error
            }),
        ),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[derive(Serialize)]
enum AuthUserError {
    NoAuthorizationHeader,
    AuthorizationParsing,
    InvalidToken,
    UserNotAllowed,
    RoleNotAllowed,
}

impl AuthUserError {
    pub fn to_http_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::UNAUTHORIZED)
            .content_type("application/json")
            .body(json!({ "error": self }).to_string())
    }
}

macro_rules! enforce_id {
    ($req:expr, $id:expr) => {
        if let Err(auth_error) = request_auth_user_enforce_id($req, $id) {
            return auth_error.to_http_response();
        }   
    };
}
use enforce_id;

fn request_auth_user_enforce_role(
    req: &HttpRequest,
    role: users::UserRole,
) -> Result<(), AuthUserError> {
    match request_auth_user(req) {
        Ok((_, user_role)) => {
            if role == user_role {
                Ok(())
            } else {
                Err(AuthUserError::RoleNotAllowed)
            }
        }
        Err(err) => Err(err),
    }
}

fn request_auth_user_enforce_id(req: &HttpRequest, id: i32) -> Result<(), AuthUserError> {
    match request_auth_user(req) {
        Ok((user_id, _)) => {
            if user_id == id {
                Ok(())
            } else {
                Err(AuthUserError::UserNotAllowed)
            }
        }
        Err(err) => Err(err),
    }
}

fn request_auth_user(req: &HttpRequest) -> Result<(i32, users::UserRole), AuthUserError> {
    match req.headers().get("Authorization") {
        Some(authorization) => match authorization.to_str() {
            Ok(token) => users::auth::claims_from_token(&token.to_owned())
                .map_err(|_| AuthUserError::InvalidToken),
            Err(_) => Err(AuthUserError::AuthorizationParsing),
        },
        None => Err(AuthUserError::NoAuthorizationHeader),
    }
}
