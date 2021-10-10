use camion::web::{server};
use std::{thread, time};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let srv = server::start(&database_url).await;
    thread::sleep(time::Duration::from_secs(10));
    srv.stop(true).await;
    Ok(())
}
