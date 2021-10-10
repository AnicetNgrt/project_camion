use serde::Serialize;
use crate::business::security;

#[derive(Serialize)]
struct UserAuthJwtClaims {
    id: i32,
    exp: usize,
}

pub fn from_id(id: i32, expiration_sec: i64) -> Result<String, ()> {
    security::jwt_create(&mut UserAuthJwtClaims { id, exp: 0 }, expiration_sec)
}

impl security::JwtClaims for UserAuthJwtClaims {
    fn set_expiration(&mut self, exp: usize) {
        self.exp = exp;
    }
}