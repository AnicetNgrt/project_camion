use super::users::utils_auth::disallow_anonymous_and_role;
use crate::core::{
    jokes::JokeTemplate,
    users::{self},
};
use actix_web::{http::StatusCode, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use super::ApiState;

#[derive(Deserialize)]
struct CreateJokeBody {
    pub joke: JokeTemplate,
}

#[post("/jokes/create")]
async fn create_joke(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    body: web::Json<CreateJokeBody>,
) -> HttpResponse {
    let claims = match disallow_anonymous_and_role(&req, users::Role::None) {
        Err(error) => return error.to_http_response(),
        Ok(claims) => claims,
    };

    let (status, body) = match users::find_by_id(claims.id, &&api_state.db_conn_pool).await {
        Ok(user) => match body
            .joke
            .insert_and_set_author(&user, &&api_state.db_conn_pool)
            .await
        {
            Ok(joke) => (StatusCode::OK, json!({ "success": true, "created_joke": joke })),
            Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
        },
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
    };

    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}