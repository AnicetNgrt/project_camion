use camion::{
    core::{db, security::password_salt_and_hash, users::UserRole},
    web::application::{Application, Config},
};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client as HttpClient, StatusCode,
};
use std::env;
use std::str::FromStr;
use uuid::Uuid;

mod auth;
mod users;

pub struct TestApp {
    pub url: String,
    pub db_conn_pool: db::DbPool,
}

pub async fn spawn_app() -> TestApp {
    // Configuration
    dotenv().ok();
    let host = "127.0.0.1";
    let database_url_root = env::var("DATABASE_URL_ROOT").expect("DATABASE_URL_ROOT not set");
    let test_database_name = Uuid::new_v4().to_string();
    let test_database_url = format!("{}{}", &database_url_root, &test_database_name);

    let config = Config {
        port: 0,
        host: host.to_owned(),
        db_url: test_database_url.clone(),
    };

    // Setup test db
    let test_db_pool = setup_test_database(&database_url_root, &test_database_name).await;

    // Running the test server and retrieving its port
    let app = Application::create(&config).await.unwrap();
    let app_port = app.port;
    let _ = tokio::spawn(app.run());
    TestApp {
        url: format!("http://{}:{}", host, app_port),
        db_conn_pool: test_db_pool,
    }
}

async fn setup_test_database(database_url_root: &String, name: &String) -> db::DbPool {
    // Creating test database
    let postgres_db_pool = db::build_pool(&format!("{}/postgres", database_url_root)).await;

    sqlx::query(&format!("CREATE DATABASE \"{}\";", name).as_str())
        .execute(&postgres_db_pool)
        .await
        .unwrap();

    // Running migrations on test database
    let test_db_pool = db::build_pool(&format!("{}/{}", database_url_root, name)).await;
    sqlx::migrate!("./migrations")
        .run(&test_db_pool)
        .await
        .unwrap();

    test_db_pool
}

#[actix_rt::test]
async fn ping_test() {
    let app = spawn_app().await;

    let res = reqwest::get(format!("{}{}", app.url, "/api/ping")).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "pong");
}

pub async fn get(
    app: &TestApp,
    route: &str,
    headers: Vec<(&str, &str)>,
) -> (StatusCode, serde_json::Value) {
    let header_map = headers_vec_to_reqwest_map(headers);

    let res = HttpClient::new()
        .get(format!("{}{}", app.url, route))
        .headers(header_map)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let body = res.text().await.unwrap();
    println!("{} : {}", status, body);
    (status, serde_json::from_str(&body).unwrap())
}

pub async fn post_json(
    app: &TestApp,
    route: &str,
    json_body: serde_json::Value,
    headers: Vec<(&str, &str)>,
) -> (StatusCode, serde_json::Value) {
    let header_map = headers_vec_to_reqwest_map(headers);

    let res = HttpClient::new()
        .post(format!("{}{}", app.url, route))
        .headers(header_map)
        .json(&json_body)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let body = res.text().await.unwrap();
    (status, serde_json::from_str(&body).unwrap())
}

pub fn headers_vec_to_reqwest_map(headers: Vec<(&str, &str)>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    for (name, value) in headers.iter() {
        header_map.append(
            HeaderName::from_str(name).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }
    header_map
}

pub async fn insert_test_user(
    username: &str,
    email: &str,
    password: &str,
    role: &UserRole,
    pool: &db::DbPool,
) -> i32 {
    let hashed_salted_password = password_salt_and_hash(&password.to_string()).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO users ( username, email, password, role )
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
    "#,
        username,
        email,
        hashed_salted_password,
        role as _
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .id
}
