use actix_web::dev::Server;
use camion::web::server;
use dotenv::dotenv;
use std::env;

async fn init() -> Server {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL not set");
    server::start(&database_url).await
}

async fn cleanup() -> Server {
    
}

#[test]
async fn 