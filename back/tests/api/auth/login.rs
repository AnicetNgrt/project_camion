use crate::api::{post_json, spawn_app, TestApp};
use serde_json::json;

use camion::core::{
    db,
    security::{password_salt_and_hash}
};

async fn try_login(
    app: &TestApp,
    login: &str,
    password: &str,
) -> (reqwest::StatusCode, serde_json::Value) {
    post_json(
        app,
        "/api/auth/login",
        json!({
            "login": login,
            "password": password
        }),
    )
    .await
}

async fn insert_test_user(username: &str, email: &str, password: &str, pool: &db::DbPool) {
    let hashed_salted_password = password_salt_and_hash(&password.to_string()).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO users ( username, email, password, role )
        VALUES ( $1, $2, $3, 'none' )
    "#,
        username,
        email,
        hashed_salted_password
    )
    .execute(pool)
    .await
    .unwrap();
}

#[actix_rt::test]
async fn success_with_same_username_and_password() {
    let app = spawn_app().await;
    let username = "Anicet";
    let email = "";
    let password = "secret_password";
    insert_test_user(username, email, password, &app.db_conn_pool).await;

    let (status_code, body) = try_login(&app, username, password).await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    match body.get("token") {
        Some(value) => assert!(value.is_string()),
        None => assert!(false)
    }
}

#[actix_rt::test]
async fn success_with_same_email_and_password() {
    let app = spawn_app().await;
    let username = "";
    let email = "test@test.fr";
    let password = "secret_password";
    insert_test_user(username, email, password, &app.db_conn_pool).await;

    let (status_code, body) = try_login(&app, email, password).await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    match body.get("token") {
        Some(value) => assert!(value.is_string()),
        None => assert!(false)
    }
}

#[actix_rt::test]
async fn tells_when_username_unknown() {
    let app = spawn_app().await;
    let (status_code, body) = try_login(&app, "superUnknownUsername", "").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    let reason = body.get("reason").unwrap();
    assert_eq!(reason, &json!("UnknownLogin"));
}

#[actix_rt::test]
async fn tells_when_username_known_and_password_wrong() {
    let app = spawn_app().await;
    let username = "Anicet";
    insert_test_user(username, "", "superPassword", &app.db_conn_pool).await;

    let (status_code, body) = try_login(&app, username, "superWrongPassword").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    let reason = body.get("reason").unwrap();
    assert_eq!(reason, &json!("InvalidPassword"));
}

#[actix_rt::test]
async fn is_ambiguous_when_email_unknown_or_email_known_and_password_wrong() {
    let app = spawn_app().await;
    let email = "test@test.fr";
    let password = "superPassword";
    insert_test_user("", email, password, &app.db_conn_pool).await;

    let (status_code, body) = try_login(&app, email, "superWrongPassword").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    let reason = body.get("reason").unwrap();
    assert_eq!(reason, &json!("InvalidCredentials"));

    let (status_code, body) = try_login(&app, "wrong@test.fr", password).await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    let reason = body.get("reason").unwrap();
    assert_eq!(reason, &json!("InvalidCredentials"));
}