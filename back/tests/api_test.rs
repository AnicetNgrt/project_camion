use camion::{
    business::{db},
    web::application::{Application, Config},
};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

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
    let test_database_url = format!("{}/{}", &database_url_root, &test_database_name);
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
        db_conn_pool: test_db_pool
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

    let resp = reqwest::get(format!("{}{}", app.url, "/api/ping"))
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    assert_eq!(body, "pong");
}

#[actix_rt::test]
async fn register_test_username_syntax_rules() {
    let app = spawn_app().await;
    todo!("test username syntax rules");
}
