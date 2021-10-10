use super::{post_json, spawn_app, TestApp};
use serde_json::json;

async fn try_register(app: &TestApp, username: &str, email: &str, password: &str) -> (reqwest::StatusCode, serde_json::Value) {
    post_json(
        app,
        "/api/auth/register",
        json!({
            "username": username,
            "email": email,
            "password": password
        }),
    ).await
}

#[actix_rt::test]
async fn register_test_password_rules() {
    {
        let app = spawn_app().await;
        let (status_code, body) = try_register(&app, "", "", "strongPassword10#[").await;
        assert_eq!(status_code, reqwest::StatusCode::OK);
        assert_eq!(body["issues"][2], serde_json::Value::Null);
    }

    {
        let app = spawn_app().await;
        let (status_code, body) = try_register(&app, "", "", "blabasecret").await;
        assert_eq!(status_code, reqwest::StatusCode::OK);
        let password_issues = body["issues"][2].as_array().unwrap();
        assert!(password_issues.iter().any(|v| *v == json!("NoNumeric")));
        assert!(password_issues.iter().any(|v| *v == json!("NoSpecialChars")));
        assert!(password_issues.iter().any(|v| *v == json!("NoUpperCase")));
    }

    {
        let app = spawn_app().await;
        let (status_code, body) = try_register(&app, "", "", "BLABLASECRET").await;
        assert_eq!(status_code, reqwest::StatusCode::OK);
        let password_issues = body["issues"][2].as_array().unwrap();
        assert!(password_issues.iter().any(|v| *v == json!("NoNumeric")));
        assert!(password_issues.iter().any(|v| *v == json!("NoSpecialChars")));
        assert!(password_issues.iter().any(|v| *v == json!("NoLowerCase")));
    }

    {
        let app = spawn_app().await;
        let (status_code, body) = try_register(&app, "", "", "1917").await;
        assert_eq!(status_code, reqwest::StatusCode::OK);
        let password_issues = body["issues"][2].as_array().unwrap();
        assert!(password_issues.iter().any(|v| *v == json!("NoAlphabetic")));
        assert!(password_issues.iter().any(|v| *v == json!("NoSpecialChars")));
        assert!(password_issues.iter().any(|v| *v == json!("NoLowerCase")));
        assert!(password_issues.iter().any(|v| *v == json!("NoUpperCase")));
        assert!(password_issues.iter().any(|v| *v == json!("NotLongEnough")));
    }

    {
        let app = spawn_app().await;
        let (status_code, body) = try_register(&app, "", "", "~#~~{{{~]]{&&#").await;
        assert_eq!(status_code, reqwest::StatusCode::OK);
        let password_issues = body["issues"][2].as_array().unwrap();
        assert!(password_issues.iter().any(|v| *v == json!("NoAlphabetic")));
        assert!(password_issues.iter().any(|v| *v == json!("NoLowerCase")));
        assert!(password_issues.iter().any(|v| *v == json!("NoUpperCase")));
        assert!(password_issues.iter().any(|v| *v == json!("NoNumeric")));
    }
}
