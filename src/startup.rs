use crate::configuration;
use crate::routes::{health_check, home};

use actix_web::dev::Server;

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
        todo!()
    }
}
