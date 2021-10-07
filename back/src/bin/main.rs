use camion::web::{server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::start().await
}
