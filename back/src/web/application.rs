use actix_web::{App, HttpServer, dev::Server};
use std::net::TcpListener;

use crate::core::{db};
use crate::web::{api};

pub struct Application {
    server: Server,
    pub port: u16,
}

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
}

impl Application {
    pub async fn create(config: &Config) -> Result<Self, std::io::Error> {
        let pool = db::build_pool(&config.db_url).await;

        let address = format!("{}:{}", config.host, config.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
    
        let server = HttpServer::new(move || {
            App::new()
                .service(api::service(pool.clone()))
        })
        .listen(listener)?
        .run();
    
        Ok(Self { port, server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
