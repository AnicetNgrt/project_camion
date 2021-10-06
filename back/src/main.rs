use actix_web::{App, HttpServer};

use camion::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(api::service())
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
