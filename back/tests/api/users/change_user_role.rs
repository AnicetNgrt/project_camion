use crate::api::{
    insert_test_user, post_json, spawn_app, users::create_user_and_login_with_username, TestApp,
};
use camion::core::users::Role;
use reqwest::StatusCode;
use serde_json::json;

async fn post_change_role_request_by_username(
    app: &TestApp,
    username: &str,
    role: &Role,
    jwt: Option<&str>,
) -> (reqwest::StatusCode, serde_json::Value) {
    let headers = match jwt {
        Some(jwt) => vec![("Authorization", jwt)],
        None => vec![]
    };
    post_json(
        app,
        &format!("/api/users/{}/role", username),
        json!({ "new_role": role }),
        headers,
    )
    .await
}

#[actix_rt::test]
async fn admins_can_change_anyones_role_to_anything() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "admin", "a0@test.fr", "pass", &Role::Admin)
            .await;
    
    insert_test_user("Anicet", "a1@test.fr", "pass", &Role::Admin, &app.db_conn_pool).await;
    
    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::Author, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::None, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::Admin, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::None, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::Author, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);
}

async fn cannot_change_anyone_role(app: TestApp, jwt: Option<&str>) {
    insert_test_user("Yoann", "a1@test.fr", "pass", &Role::None, &app.db_conn_pool).await;
    insert_test_user("Anicet", "a2@test.fr", "pass", &Role::Author, &app.db_conn_pool).await;
    insert_test_user("Félicie", "a3@test.fr", "pass", &Role::Admin, &app.db_conn_pool).await;
    
    let (status_code, _) = post_change_role_request_by_username(&app, "Yoann", &Role::Admin, jwt).await;
    assert_eq!(status_code, StatusCode::UNAUTHORIZED);

    let (status_code, _) = post_change_role_request_by_username(&app, "Anicet", &Role::None, jwt).await;
    assert_eq!(status_code, StatusCode::UNAUTHORIZED);

    let (status_code, _) = post_change_role_request_by_username(&app, "Félicie", &Role::Author, jwt).await;
    assert_eq!(status_code, StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn authors_cannot_change_anyones_role() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "author", "a0@test.fr", "pass", &Role::Author)
            .await;
    
    cannot_change_anyone_role(app, Some(&jwt)).await;
}

#[actix_rt::test]
async fn none_cannot_change_anyones_role() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "noneuser", "a0@test.fr", "pass", &Role::None)
            .await;
    
    cannot_change_anyone_role(app, Some(&jwt)).await;
}

#[actix_rt::test]
async fn anonymous_cannot_change_anyones_role() {
    let app = spawn_app().await;
    
    cannot_change_anyone_role(app, None).await;
}