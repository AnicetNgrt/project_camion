use crate::business::db;

pub async fn insert_returning_id(
    username: &String,
    email: &String,
    password: &String,
    pool: &db::DbPool,
) -> Result<i32, sqlx::Error> {
    let record = sqlx::query!(
        r#"
    INSERT INTO users ( username, email, password )
    VALUES ( $1, $2, $3 )
    RETURNING id
    "#,
        username,
        email,
        password
    )
    .fetch_one(pool)
    .await?;

    Ok(record.id)
}
