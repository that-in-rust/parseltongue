/**
 * Application Pyramid:
 * L1: Core server setup
 * L2: Database connection
 * L3: Route handlers
 * L4: Error management
 */
use actix_web::{web, App, HttpServer, middleware};
use mongodb::Client;
use crate::db::AnalysisStore;
use crate::analysis::AnalysisEngine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let mongo_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");
    
    let db = mongo_client.database("parseltongue");
    let store = AnalysisStore::new(db.clone()).await;
    let engine = AnalysisEngine::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(store.clone()))
            .app_data(web::Data::new(engine.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/analyze").route(web::post().to(handlers::start_analysis)))
                    .service(web::resource("/status/{job_id}").route(web::get().to(handlers::get_status)))
                    .service(web::resource("/results/{job_id}").route(web::get().to(handlers::get_results)))
                    .service(web::resource("/aggregate").route(web::get().to(handlers::get_performance_stats)))
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
} 