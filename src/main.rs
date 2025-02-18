mod config;
mod router;
mod controller;
mod model;
mod dto;
mod utils;


use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use mongodb::{Client as MongoClient, options::ClientOptions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::init_config().await;


    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mongo_client_options = ClientOptions::parse(&config.db.url).await.unwrap();
    let mongo_client = MongoClient::with_options(mongo_client_options).unwrap();

    let server_host = config.server.host.clone();
    let server_port = config.server.port;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::new("%{r}a %r %s"))
            .configure(router::config)
    });
    server.bind(server_host + ":" + &server_port.to_string())?.run().await
}