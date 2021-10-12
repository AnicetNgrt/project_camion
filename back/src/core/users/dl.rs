use super::{UserRole};
use crate::core::db;

macro_rules! find_by_x {
    ($field:literal, $value:ident, $pool:ident) => {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password, role as "role: UserRole"
            FROM users WHERE "# + $field + r#" = $1 
            "#,
            $value
        )
        .fetch_one($pool)
        .await
    };
}
pub(crate) use find_by_x;

pub async fn insert_returning_id(
    username: &String,
    email: &String,
    password: &String,
    role: UserRole,
    pool: &db::DbPool,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
    INSERT INTO users ( username, email, password, role )
    VALUES ( $1, $2, $3, $4 )
    RETURNING id
    "#,
        username,
        email,
        password,
        role as _
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}