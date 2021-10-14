use camion::core::users::Role;
use crate::api::{post_json, spawn_app, insert_test_user, TestApp};
use serde_json::json;

use camion::core::security::{password_verify, password_salt_and_hash};

async fn register(
    app: &TestApp,
    username: &str,
    email: &str,
    password: &str,
) -> (reqwest::StatusCode, serde_json::Value) {
    post_json(
        app,
        "/api/auth/register",
        json!({
            "username": username,
            "email": email,
            "password": password
        }),
        vec![]
    )
    .await
}

#[actix_rt::test]
async fn registers_using_same_username_email_and_hashed_salted_password() {
    let app = spawn_app().await;
    let password = "superPass2021'-";
    let (status_code, body) = register(&app, "Anicet", "test@gmail.com", password).await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);
    
    let res = sqlx::query!(r#"
    SELECT password FROM users
    WHERE username = $1 AND email = $2
    "#, "Anicet", "test@gmail.com")
    .fetch_one(&app.db_conn_pool)
    .await;

    match res {
        Ok(record) => {
            assert!(password_verify(&password.to_string(), &record.password));
            assert_ne!(record.password, password_salt_and_hash(&password.to_string()).unwrap());
        },
        Err(_) => assert!(false)
    };
}

#[actix_rt::test]
async fn email_find_no_issues_valid() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "abc_def@mail.cc", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);

    let (status_code, body) = register(&app, "", "abc@mail-archive.com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);

    let (status_code, body) = register(&app, "", "abc.def@mail.org", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);

    let (status_code, body) = register(&app, "", "abc-d@mail.com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);

    let (status_code, body) = register(&app, "", "abc-@mail.com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);

    let (status_code, body) = register(&app, "", "abc#def@mail.com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["email"], serde_json::Value::Null);
}

#[actix_rt::test]
async fn email_reject_malformed() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "hello", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["email"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("Malformed")));

    let (status_code, body) = register(&app, "", "HELLO", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["email"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("Malformed")));

    let (status_code, body) = register(&app, "", "abc@", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["email"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("Malformed")));

    let (status_code, body) = register(&app, "", "abc@mail..com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["email"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("Malformed")));
}

#[actix_rt::test]
async fn email_reject_not_unique() {
    let app = spawn_app().await;
    sqlx::query!(
        r#"
        INSERT INTO users ( username, email, password, role )
        VALUES ( $1, $2, $3, 2 )
    "#,
        "test",
        "anicet@gmail.com",
        "test"
    )
    .execute(&app.db_conn_pool)
    .await
    .unwrap();

    let (status_code, body) = register(&app, "", "anicet@gmail.com", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["email"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NotUnique")));
}

#[actix_rt::test]
async fn username_find_no_issues_valid() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "Félicie", "", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["username"], serde_json::Value::Null);
}

#[actix_rt::test]
async fn username_reject_too_short() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "An", "", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["username"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("TooShort")));
}

#[actix_rt::test]
async fn username_reject_too_long() {
    let app = spawn_app().await;
    let (status_code, body) =
    register(&app, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["username"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("TooLong")));
}

#[actix_rt::test]
async fn username_reject_email_like() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "truc.machin@machin-bidule.fr", "", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["username"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("EmailLike")));
}

#[actix_rt::test]
async fn username_reject_not_unique() {
    let app = spawn_app().await;
    let username = "Anicet";
    insert_test_user(username, "", "", &Role::None, &app.db_conn_pool).await;
    let (status_code, body) = register(&app, username, "", "").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["username"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NotUnique")));
}

#[actix_rt::test]
async fn password_find_no_issues_valid() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "", "strongPassword10#[").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    assert_eq!(body["issues"]["password"], serde_json::Value::Null);
}

#[actix_rt::test]
async fn password_reject_only_lowercase_alphabetic() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "", "blabasecret").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["password"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NoNumeric")));
    assert!(issues.iter().any(|v| *v == json!("NoSpecialChars")));
    assert!(issues.iter().any(|v| *v == json!("NoUpperCase")));
}

#[actix_rt::test]
async fn password_reject_only_uppercase_alphabetic() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "", "BLABLASECRET").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["password"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NoNumeric")));
    assert!(issues.iter().any(|v| *v == json!("NoSpecialChars")));
    assert!(issues.iter().any(|v| *v == json!("NoLowerCase")));
}

#[actix_rt::test]
async fn password_reject_only_numeric_or_short() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "", "1917").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["password"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NoAlphabetic")));
    assert!(issues.iter().any(|v| *v == json!("NoSpecialChars")));
    assert!(issues.iter().any(|v| *v == json!("NoLowerCase")));
    assert!(issues.iter().any(|v| *v == json!("NoUpperCase")));
    assert!(issues.iter().any(|v| *v == json!("NotLongEnough")));
}

#[actix_rt::test]
async fn password_reject_only_special_chars() {
    let app = spawn_app().await;
    let (status_code, body) = register(&app, "", "", "~#~~{{{~]]{&&#").await;
    assert_eq!(status_code, reqwest::StatusCode::OK);
    let issues = body["issues"]["password"].as_array().unwrap();
    assert!(issues.iter().any(|v| *v == json!("NoAlphabetic")));
    assert!(issues.iter().any(|v| *v == json!("NoLowerCase")));
    assert!(issues.iter().any(|v| *v == json!("NoUpperCase")));
    assert!(issues.iter().any(|v| *v == json!("NoNumeric")));
}
