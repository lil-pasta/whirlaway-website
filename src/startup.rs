use crate::configuration;
use crate::routes::{health_check, home};

use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(conf: configuration::Settings) -> Result<Self, anyhow::Error> {
        let address = conf.application_address();
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener).await?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stop(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/static/css", "static/css/")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
            .service(Files::new("/static/images", "static/images").use_last_modified(true))
            .service(Files::new("/static/html", "static/html").use_last_modified(true))
            .service(health_check)
            .service(home)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
