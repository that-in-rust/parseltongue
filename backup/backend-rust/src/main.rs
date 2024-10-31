/**
 * Application Pyramid:
 * L1: Core server setup
 * L2: Configuration loading
 * L3: Service initialization
 * L4: Route definitions
 */

mod analysis;
mod config;
mod db;
mod handlers;

use crate::analysis::AnalysisEngine;
use crate::config::Config;
use crate::db::AnalysisStore;
use actix_web::{middleware, web, App, HttpServer};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // L1: Core server setup
    env_logger::init();

    // L2: Configuration loading
    let config = Config::from_env();
    config.validate().expect("Invalid configuration");

    // L3: Service initialization
    let mongo_client = Client::with_uri_str(&config.mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db = mongo_client.database("parseltongue");
    let store = AnalysisStore::new(db.clone()).await;
    let engine = AnalysisEngine::new(store.clone(), config.clone());

    // L4: Route definitions
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(store.clone()))
            .app_data(web::Data::new(engine.clone()))
            .configure(handlers::init_routes)
    })
    .bind(format!("127.0.0.1:{}", config.port))?
    .run()
    .await
} 