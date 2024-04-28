use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use config::config;
use routes::{generic, targeted};

mod config;
mod error;
mod routes;
mod validation;

pub struct WebhookState {
    pub processed_deliveries: Vec<String>,
}
pub type State = Mutex<WebhookState>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting webhook on 127.0.0.1:4000");

    // Load the config
    config();

    HttpServer::new(|| {
        App::new()
            .service(generic)
            .service(targeted)
            .app_data(web::Data::new(WebhookState {
                processed_deliveries: vec![],
            }))
    })
    .bind(("127.0.0.1", 4001))?
    .run()
    .await
}
