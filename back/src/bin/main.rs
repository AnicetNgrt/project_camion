use camion::web::{application::{Application, Config}};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let config = Config {
        db_url,
        host: "127.0.0.1".to_owned(),
        port: 8080
    };
    let app = Application::create(&config).await?;
    app.run().await
}
