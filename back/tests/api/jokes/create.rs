use crate::api::{
    post_json, spawn_app, users::create_user_and_login_with_username, TestApp,
};
use camion::core::users::Role;
use reqwest::StatusCode;
use serde_json::json;

async fn post_create_joke_request(
    app: &TestApp,
    joke_json: serde_json::Value,
    jwt: Option<&str>,
) -> (reqwest::StatusCode, serde_json::Value) {
    let headers = match jwt {
        Some(jwt) => vec![("Authorization", jwt)],
        None => vec![]
    };
    post_json(
        app,
        &format!("/api/jokes"),
        json!({
            "joke": joke_json
        }),
        headers,
    )
    .await
}

#[actix_rt::test]
async fn admins_can_create_jokes() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "admin", "a0@test.fr", "pass", &Role::Admin)
            .await;

    let (status_code, _) = post_create_joke_request(&app, json!({}), Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[actix_rt::test]
async fn authors_can_create_jokes() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "speaker", "a0@test.fr", "pass", &Role::Author)
            .await;

    let (status_code, _) = post_create_joke_request(&app, json!({}), Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[actix_rt::test]
async fn none_cannot_create_jokes() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "noneuser", "a0@test.fr", "pass", &Role::None)
            .await;

    let (status_code, _) = post_create_joke_request(&app, json!({}), Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn anonymous_cannot_create_jokes() {
    let app = spawn_app().await;

    let (status_code, _) = post_create_joke_request(&app, json!({}), None).await;
    assert_eq!(status_code, StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn creates_valid_jokes_returning_id() {
    let app = spawn_app().await;

    let (_, jwt) = 
        create_user_and_login_with_username(&app, "admin", "a0@test.fr", "pass", &Role::Admin)
            .await;

    let joke = json!({
        "title": "Test",
        "lines": [
            { "speaker": "Author1", "content": "blabl abalab la balabba abalba" },
            { "speaker": "Ahor3", "content": "blabl abaladzadazhalabba abalba" },
            { "speaker": "Aut862or2", "content": "blabl ab abalba" },
            { "speaker": "Author8", "content": "blabl abalab laaza87131 abalba" },
            { "speaker": "Auth", "content": "blabl alabba abalba" },
            { "speaker": "Author1", "content": "blabl abalab la^aa$$ba abalba" }
        ]
    });

    let (status_code, body) = post_create_joke_request(&app, joke, Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(body["success"].as_bool().unwrap(), true);
    assert!(body["created_joke"]["id"].as_i64().is_some());
}
