/**
 * Database Layer Pyramid:
 * L1: Core database operations
 * L2: Model definitions
 * L3: Async interactions
 * L4: Error handling
 */

use mongodb::{bson::doc, bson::Document, Client, Database, Collection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisJob {
    // L2: Model definitions
    pub job_id: String,
    pub backend: String,
    pub status: String,
    pub stage: String,
    pub current_file: Option<String>,
    pub progress: f64,
    pub language_breakdown: Option<HashMap<String, i32>>,
    pub processing_time_ms: Option<i64>,
    pub files_per_second: Option<f64>,
    pub total_files: Option<i32>,
}

#[derive(Clone)]
pub struct AnalysisStore {
    // L1: Core database operations
    collection: Collection<AnalysisJob>,
}

impl AnalysisStore {
    pub async fn new(db: Database) -> Self {
        let collection = db.collection::<AnalysisJob>("analysis_jobs");
        AnalysisStore { collection }
    }

    // L3: Async interactions
    pub async fn insert_job(&self, job: AnalysisJob) -> mongodb::error::Result<()> {
        self.collection.insert_one(job, None).await.map(|_| ())
    }

    pub async fn update_job(&self, job_id: &str, update_doc: Document) -> mongodb::error::Result<()> {
        self.collection
            .update_one(doc! { "job_id": job_id }, update_doc, None)
            .await
            .map(|_| ())
    }

    pub async fn get_job(&self, job_id: &str) -> mongodb::error::Result<Option<AnalysisJob>> {
        self.collection.find_one(doc! { "job_id": job_id }, None).await
    }

    // L4: Error handling is inherent in the Result types
} 