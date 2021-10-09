use camion::web::{server};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    server::start().await;

    Ok(())
}
