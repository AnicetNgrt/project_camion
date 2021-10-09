use actix_web::{App, HttpServer};

use crate::business::{db};
use crate::web::{api};

pub async fn start() {
    let pool = db::build_pool().await;

    HttpServer::new(move || {
        App::new()
            .service(api::service(pool.clone()))
    })
        .bind("127.0.0.1:8080")
        .expect("bind issue")
        .run()
        .await
        .expect("server didn't start");

    println!("HERE");
}
