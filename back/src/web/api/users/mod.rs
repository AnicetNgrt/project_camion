use crate::core::users::{self, Role};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use self::utils_auth::enforce_role;

use super::ApiState;

pub mod utils_auth;

#[derive(Deserialize)]
struct SearchUserQuery {
    pub query: String,
}

#[post("/users/search")]
async fn search_users(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    query: web::Json<SearchUserQuery>,
) -> HttpResponse {
    let claims = utils_auth::auth_user(&req);

    let claims = match claims {
        Ok(claims) => Some(claims),
        Err(utils_auth::Error::NoAuthorizationHeader) => None,
        Err(error) => return error.to_http_response(),
    };

    let (status, body) = match users::search_by_username_to_json_as_seen_from(
        &query.query,
        &claims,
        &&api_state.db_conn_pool,
    )
    .await
    {
        Ok(users) => (StatusCode::OK, json!({ "results": users })),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
    };

    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[get("/users/{username}")]
async fn get_user_data(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let claims = utils_auth::auth_user(&req);

    let claims = match claims {
        Ok(claims) => Some(claims),
        Err(utils_auth::Error::NoAuthorizationHeader) => None,
        Err(error) => return error.to_http_response(),
    };

    let (status, body) = match users::find_by_username(&path.0, &&api_state.db_conn_pool).await {
        Ok(user) => (StatusCode::OK, user.to_json_as_seen_from(&claims)),
        Err(users::Error::NotFound) => (StatusCode::NOT_FOUND, json!({})),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
    };

    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}

#[derive(Deserialize)]
struct ChangeRoleBody {
    pub new_role: Role,
}

#[post("/users/{username}/role")]
async fn change_user_role(
    req: HttpRequest,
    api_state: web::Data<ApiState>,
    body: web::Json<ChangeRoleBody>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    if let Err(error) = enforce_role(&req, users::Role::Admin) {
        return error.to_http_response();
    }
    let (status, body) = match users::find_by_username(&path.0, &&api_state.db_conn_pool).await {
        Ok(mut user) => {
            match user.set_role(body.new_role, &&api_state.db_conn_pool).await {
                Ok(_) => (StatusCode::OK, json!({})),
                Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })) 
            }
        },
        Err(users::Error::NotFound) => (StatusCode::NOT_FOUND, json!({})),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": error })),
    };

    HttpResponse::build(status)
        .content_type("application/json")
        .body(body.to_string())
}