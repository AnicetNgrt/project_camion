use serde::{Serialize, Deserialize};

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Role {
    Admin,
    Author,
    None,
}

impl From<i32> for Role {
    fn from(i: i32) -> Self {
        match i {
            0 => Role::Admin,
            1 => Role::Author,
            _ => Role::None
        }
    }
}