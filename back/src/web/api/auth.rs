use crate::core::{users};
use actix_web::{http::StatusCode, post, web, HttpResponse};
use serde_json::json;

use super::{ApiState};

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
        Ok(jwt) => (StatusCode::OK, json!({ "token": jwt })),
        Err(users::login::Error::Denied(reason)) => {
            (StatusCode::UNAUTHORIZED, json!({ "reason": reason }))
        },
        Err(users::login::Error::Failure(failure)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "error": failure }),
        ),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}