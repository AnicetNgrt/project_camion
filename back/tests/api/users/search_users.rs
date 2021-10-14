use crate::api::{
    insert_test_user, post_json, spawn_app, users::create_user_and_login_with_username, TestApp,
};
use camion::core::users::Role;
use reqwest::StatusCode;
use serde_json::json;

async fn post_search_request_by_username(
    app: &TestApp,
    query: &str,
    jwt: Option<&str>,
) -> (reqwest::StatusCode, serde_json::Value) {
    let headers = match jwt {
        Some(jwt) => vec![("Authorization", jwt)],
        None => vec![]
    };
    post_json(
        app,
        &format!("/api/users/search"),
        json!({ "query": query }),
        headers,
    )
    .await
}

fn contains_username(results: &Vec<serde_json::Value>, username: &str) -> bool {
    results
        .iter()
        .any(|r| r["username"].as_str().unwrap() == username)
}

async fn insert_test_users(app: &TestApp) {
    let users = vec![
        ("TrucMachin", "a1@test.fr", "", &Role::Author),
        ("BiduleChouette", "a2@test.fr", "", &Role::Author),
        ("MachtruChou", "a3@test.fr", "", &Role::Admin),
        ("biducHin", "a4@test.fr", "", &Role::None),
        ("chouetteTruc", "a5@test.fr", "", &Role::None),
    ];

    for (username, email, password, role) in users.iter() {
        insert_test_user(username, email, password, role, &app.db_conn_pool).await;
    }
}

#[actix_rt::test]
async fn returns_all_users_with_query_in_username_as_admin() {
    let app = spawn_app().await;
    let (_, jwt) =
        create_user_and_login_with_username(&app, "admin", "a0@test.fr", "pass", &Role::Admin)
            .await;
    insert_test_users(&app).await;

    let (status_code, body) = post_search_request_by_username(&app, "rUc", Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let results = body["results"].as_array().unwrap();
    assert!(contains_username(&results, "TrucMachin"));
    assert!(contains_username(&results, "MachtruChou"));
    assert!(contains_username(&results, "chouetteTruc"));
    assert_eq!(results.len(), 3);
}

#[actix_rt::test]
async fn returns_authors_and_admins_only_with_query_in_username_as_author() {
    let app = spawn_app().await;
    let (_, jwt) =
        create_user_and_login_with_username(&app, "author", "a0@test.fr", "pass", &Role::Author)
            .await;
    insert_test_users(&app).await;

    let (status_code, body) = post_search_request_by_username(&app, "rUc", Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let results = body["results"].as_array().unwrap();
    assert!(contains_username(&results, "TrucMachin"));
    assert!(contains_username(&results, "MachtruChou"));
    assert_eq!(results.len(), 2);
}

#[actix_rt::test]
async fn returns_authors_and_admins_only_with_query_in_username_as_none() {
    let app = spawn_app().await;
    let (_, jwt) =
        create_user_and_login_with_username(&app, "person", "a0@test.fr", "pass", &Role::None)
            .await;
    insert_test_users(&app).await;

    let (status_code, body) = post_search_request_by_username(&app, "rUc", Some(&jwt)).await;
    assert_eq!(status_code, StatusCode::OK);

    let results = body["results"].as_array().unwrap();
    assert!(contains_username(&results, "TrucMachin"));
    assert!(contains_username(&results, "MachtruChou"));
    assert_eq!(results.len(), 2);
}

#[actix_rt::test]
async fn returns_authors_and_admins_only_with_query_in_username_as_anonymous() {
    let app = spawn_app().await;

    insert_test_users(&app).await;

    let (status_code, body) = post_search_request_by_username(&app, "rUc", None).await;
    assert_eq!(status_code, StatusCode::OK);

    let results = body["results"].as_array().unwrap();
    assert!(contains_username(&results, "TrucMachin"));
    assert!(contains_username(&results, "MachtruChou"));
    assert_eq!(results.len(), 2);
}
