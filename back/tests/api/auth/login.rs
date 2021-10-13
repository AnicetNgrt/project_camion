use camion::core::users::UserRole;
use crate::api::{post_json, spawn_app, insert_test_user, TestApp};
use serde_json::json;

pub async fn login(
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
        vec![]
    )
    .await
}

#[actix_rt::test]
async fn success_with_same_username_and_password() {
    let app = spawn_app().await;
    let username = "Anicet";
    let email = "";
    let password = "secret_password";
    insert_test_user(username, email, password, &UserRole::None, &app.db_conn_pool).await;

    let (status_code, body) = login(&app, username, password).await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_ne!(body["token"].to_string(), "null");
}

#[actix_rt::test]
async fn success_with_same_email_and_password() {
    let app = spawn_app().await;
    let username = "";
    let email = "test@test.fr";
    let password = "secret_password";
    insert_test_user(username, email, password, &UserRole::None, &app.db_conn_pool).await;

    let (status_code, body) = login(&app, email, password).await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_ne!(body["token"].to_string(), "null");
}

#[actix_rt::test]
async fn tells_when_username_unknown() {
    let app = spawn_app().await;
    let (status_code, body) = login(&app, "superUnknownUsername", "").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    assert_eq!(body["reason"], json!("UnknownLogin"));
}

#[actix_rt::test]
async fn tells_when_username_known_and_password_wrong() {
    let app = spawn_app().await;
    let username = "Anicet";
    insert_test_user(username, "", "superPassword", &UserRole::None, &app.db_conn_pool).await;

    let (status_code, body) = login(&app, username, "superWrongPassword").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    assert_eq!(body["reason"], json!("InvalidPassword"));
}

#[actix_rt::test]
async fn is_ambiguous_when_email_unknown_or_email_known_and_password_wrong() {
    let app = spawn_app().await;
    let email = "test@test.fr";
    let password = "superPassword";
    insert_test_user("", email, password, &UserRole::None, &app.db_conn_pool).await;

    let (status_code, body) = login(&app, email, "superWrongPassword").await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    assert_eq!(body["reason"], json!("InvalidCredentials"));

    let (status_code, body) = login(&app, "wrong@test.fr", password).await;
    assert_eq!(status_code, reqwest::StatusCode::UNAUTHORIZED);
    assert_eq!(body["reason"], json!("InvalidCredentials"));
}