use super::{User, Role};
use crate::core::db;

pub struct UserPostgres {
    pub id: i32,
    pub username: String,
    pub role: i32,
    pub email: String,
    pub password: String,
}

impl Into<User> for UserPostgres {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            role: Role::from(self.role)
        }
    }
}

macro_rules! find_by_x {
    ($field:literal, $value:ident, $pool:ident) => {
        sqlx::query_as!(
            dl::UserPostgres,
            r#"
            SELECT id, username, email, password, role
            FROM users WHERE "# + $field + r#" = $1 
            "#,
            $value
        )
        .fetch_one($pool)
        .await
        .map(|user| user.into())
    };
}
pub(crate) use find_by_x;

pub async fn update_role_returning_id(
    id: i32,
    new_role: Role,
    pool: &db::DbPool,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
    UPDATE users
    SET role = $1
    WHERE id = $2
    RETURNING *
    "#,
        new_role as i32,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}

pub async fn search_by_username(
    query: &String,
    pool: &db::DbPool,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        UserPostgres,
        r#"
        SELECT id, username, email, password, role
        FROM users WHERE username ~* $1
        "#,
        query
    )
    .fetch_all(pool)
    .await
    .map(|records| {
        records
            .into_iter()
            .map(|user| user.into())
            .collect()
    })
}

pub async fn insert_returning_id(
    username: &String,
    email: &String,
    password: &String,
    role: Role,
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
        role as i32
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}
