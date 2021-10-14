use crate::core::{
    users::{token, Role},
};
use actix_web::{http::StatusCode, HttpRequest, HttpResponse};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum Error {
    NoAuthorizationHeader,
    AuthorizationParsing,
    InvalidToken,
    UserNotAllowed,
    RoleNotAllowed,
}

impl Error {
    pub fn to_http_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::UNAUTHORIZED)
            .content_type("application/json")
            .body(json!({ "error": self }).to_string())
    }
}

pub fn enforce_role(
    req: &HttpRequest,
    role: Role,
) -> Result<(), Error> {
    match auth_user(req) {
        Ok(token::Claims{ role: user_role, ..}) => {
            if role == user_role {
                Ok(())
            } else {
                Err(Error::RoleNotAllowed)
            }
        }
        Err(err) => Err(err),
    }
}

pub fn enforce_id(req: &HttpRequest, id: i32) -> Result<(), Error> {
    match auth_user(req) {
        Ok(token::Claims{ id: user_id, ..}) => {
            if user_id == id {
                Ok(())
            } else {
                Err(Error::UserNotAllowed)
            }
        }
        Err(err) => Err(err),
    }
}

pub fn auth_user(req: &HttpRequest) -> Result<token::Claims, Error> {
    match req.headers().get("Authorization") {
        Some(authorization) => match authorization.to_str() {
            Ok(token) => token::to_claims(&token.to_owned())
                .map_err(|_| Error::InvalidToken),
            Err(_) => Err(Error::AuthorizationParsing),
        },
        None => Err(Error::NoAuthorizationHeader),
    }
}