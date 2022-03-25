use crate::configuration;
use crate::routes::{health_check, home};

use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppData {
    pub template: Arc<tera::Tera>,
}

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
    let tera = Arc::new(match tera::Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("{} error parsing templates. Exiting program", e);
            std::process::exit(1);
        }
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                template: tera.clone(),
            }))
            .service(
                Files::new("/static/css", "static/css/")
                    .prefer_utf8(true)
                    .use_last_modified(true),
            )
            .service(health_check)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
