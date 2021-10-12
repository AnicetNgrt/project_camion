use crate::core::{users};
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse};
use serde_json::json;

use super::{ApiState};

pub mod request_auth;

#[get("/user/{id}")]
async fn user_detail(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    path: web::Path<(i32,)>,
) -> HttpResponse {
    if let Err(auth_error) = request_auth::enforce_id(&req, path.0) {
        return auth_error.to_http_response()
    }
    
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