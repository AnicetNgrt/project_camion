use crate::api::{auth::login::login, get, insert_test_user, spawn_app, TestApp};
use camion::core::users::UserRole;
use reqwest::StatusCode;
use serde_json::json;

async fn get_user_data_from_username(
    app: &TestApp,
    username: &str,
    jwt: &str,
) -> (reqwest::StatusCode, serde_json::Value) {
    get(
        app,
        &format!("/api/users/{}", username),
        vec![("Authorization", jwt)],
    )
    .await
}

async fn create_user_and_login_with_username(
    app: &TestApp,
    username: &str,
    email: &str,
    password: &str,
    role: &UserRole,
) -> (i32, String) {
    let id = insert_test_user(username, email, password, role, &app.db_conn_pool).await;
    let (_, body) = login(app, username, password).await;
    (id, body["token"].as_str().unwrap().to_owned())
}

#[actix_rt::test]
async fn can_get_own_public_and_private_data_from_username() {
    let app = spawn_app().await;
    let username = "Anicet";
    let email = "test@test.fr";
    let role = UserRole::None;
    let (id, jwt) =
        create_user_and_login_with_username(&app, username, email, "superPassword", &role).await;

    let (status_code, body) = get_user_data_from_username(&app, username, &jwt).await;
    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(body["id"], json!(id));
    assert_eq!(body["role"], json!(role));
    assert_eq!(body["username"], json!(username));
    assert_eq!(body["email"], json!(email));
    assert_eq!(body["password"], serde_json::Value::Null);
}

#[actix_rt::test]
async fn can_get_anyones_public_and_private_data_from_username_as_admin() {
    let app = spawn_app().await;

    let (_, jwt) = create_user_and_login_with_username(
        &app,
        "Félicie",
        "felicie@test.fr",
        "superPassword",
        &UserRole::Admin,
    )
    .await;

    {
        let username = "Jean";
        let email = "jean@test.fr";
        let role = UserRole::None;
        let id = insert_test_user(username, email, "superPassword", &role, &app.db_conn_pool).await;

        let (status_code, body) = get_user_data_from_username(&app, username, &jwt).await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(body["id"], json!(id));
        assert_eq!(body["role"], json!(role));
        assert_eq!(body["username"], json!(username));
        assert_eq!(body["email"], json!(email));
        assert_eq!(body["password"], serde_json::Value::Null);
    }

    {
        let username = "Anicet";
        let email = "anicet@test.fr";
        let role = UserRole::Admin;
        let id = insert_test_user(username, email, "superPassword", &role, &app.db_conn_pool).await;

        let (status_code, body) = get_user_data_from_username(&app, username, &jwt).await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(body["id"], json!(id));
        assert_eq!(body["role"], json!(role));
        assert_eq!(body["username"], json!(username));
        assert_eq!(body["email"], json!(email));
        assert_eq!(body["password"], serde_json::Value::Null);
    }
}

#[actix_rt::test]
async fn can_only_get_someone_else_public_data_by_default_from_username() {
    let app = spawn_app().await;

    let (_, jwt) = create_user_and_login_with_username(
        &app,
        "Anicet",
        "anicet@test.fr",
        "superPassword",
        &UserRole::None,
    )
    .await;

    let username = "Jean";
    let email = "jean@test.fr";
    let role = UserRole::None;
    let id = insert_test_user(username, email, "superPassword", &role, &app.db_conn_pool).await;

    let (status_code, body) = get_user_data_from_username(&app, username, &jwt).await;
    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(body["id"], json!(id));
    assert_eq!(body["role"], json!(role));
    assert_eq!(body["username"], json!(username));
    assert_eq!(body["email"], serde_json::Value::Null);
    assert_eq!(body["password"], serde_json::Value::Null);
}

#[actix_rt::test]
async fn can_only_get_someone_public_data_by_default_from_username_when_anonymous() {
    let app = spawn_app().await;

    let username = "Jean";
    let email = "jean@test.fr";
    let role = UserRole::None;
    let id = insert_test_user(username, email, "superPassword", &role, &app.db_conn_pool).await;

    let (status_code, body) = get_user_data_from_username(&app, username, "").await;
    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(body["id"], json!(id));
    assert_eq!(body["role"], json!(role));
    assert_eq!(body["username"], json!(username));
    assert_eq!(body["email"], serde_json::Value::Null);
    assert_eq!(body["password"], serde_json::Value::Null);
}
