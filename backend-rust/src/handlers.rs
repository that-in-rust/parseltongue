/**
 * Handlers Pyramid:
 * L1: Core request handlers
 * L2: Status management
 * L3: Results processing
 * L4: Error handling
 */

use actix_web::{web, HttpResponse, Error};
use serde_json::json;
use crate::analysis::AnalysisEngine;
use crate::db::AnalysisStore;

pub async fn start_analysis(
    db: web::Data<AnalysisStore>,
    engine: web::Data<AnalysisEngine>,
) -> Result<HttpResponse, Error> {
    let job_id = uuid::Uuid::new_v4().to_string();
    
    // Start analysis in background
    let engine = engine.clone();
    tokio::spawn(async move {
        if let Err(e) = engine.start_analysis(&job_id).await {
            log::error!("Analysis failed: {}", e);
            db.update_error(&job_id, &e.to_string()).await.ok();
        }
    });

    Ok(HttpResponse::Ok().json(json!({ 
        "jobId": job_id,
        "status": "queued",
        "backend": "rust"
    })))
}

pub async fn get_status(
    job_id: web::Path<String>,
    db: web::Data<AnalysisStore>,
) -> Result<HttpResponse, Error> {
    match db.get_job_status(&job_id).await? {
        Some(status) => Ok(HttpResponse::Ok().json(status)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn get_results(
    job_id: web::Path<String>,
    db: web::Data<AnalysisStore>,
) -> Result<HttpResponse, Error> {
    match db.get_job_results(&job_id).await? {
        Some(results) => Ok(HttpResponse::Ok().json(results)),
        None => Ok(HttpResponse::NotFound().finish())
    }
} 