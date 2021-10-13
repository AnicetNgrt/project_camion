use crate::core::users::{self};
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse};
use serde_json::json;

use super::ApiState;

pub mod utils_auth;

#[get("/users/{username}")]
async fn user_from_username(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let claims = utils_auth::auth_user(&req);
    let finding = users::find_by_username(&path.0, &&api_state.db_conn_pool).await;
    
    let (status, body) = match (claims, finding) {
        (Ok(claims), Ok(user)) => (StatusCode::OK, user.to_json_as_seen_from(Some(claims))),
        (Err(auth_error), Ok(user)) => (StatusCode::OK, user.to_json_as_seen_from(None)),
        (_, Err(users::Error::NotFound)) => (StatusCode::NOT_FOUND, json!({})),
        (_, Err(error)) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
    };
    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}
