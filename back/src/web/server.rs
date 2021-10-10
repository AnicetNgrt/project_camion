use actix_web::{App, HttpServer, dev::Server};

use crate::business::{db};
use crate::web::{api};

pub async fn start(database_url: &String) -> Server {
    let pool = db::build_pool(database_url).await;

    HttpServer::new(move || {
        App::new()
            .service(api::service(pool.clone()))
    })
    .bind("127.0.0.1:8080")
    .expect("bind issue")
    .run()
}
