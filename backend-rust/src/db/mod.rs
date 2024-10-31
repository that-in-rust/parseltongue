/**
 * Database Layer Pyramid:
 * L1: Core database operations
 * L2: Query builders
 * L3: Aggregation pipelines
 * L4: Performance optimizations
 */

use mongodb::{
    bson::{doc, Document},
    Collection, Database,
    options::FindOneAndUpdateOptions,
};
use futures::TryStreamExt;

pub struct AnalysisStore {
    collection: Collection<Document>,
}

impl AnalysisStore {
    pub async fn new(db: Database) -> Self {
        Self {
            collection: db.collection("analysis_jobs"),
        }
    }

    pub async fn create_job(&self, job_id: &str) -> Result<(), mongodb::error::Error> {
        self.collection
            .insert_one(
                doc! {
                    "jobId": job_id,
                    "status": "queued",
                    "progress": 0.0,
                    "createdAt": chrono::Utc::now(),
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn update_progress(
        &self,
        job_id: &str,
        progress: f64,
        current_file: &str,
    ) -> Result<(), mongodb::error::Error> {
        self.collection
            .update_one(
                doc! { "jobId": job_id },
                doc! {
                    "$set": {
                        "progress": progress,
                        "currentFile": current_file,
                        "updatedAt": chrono::Utc::now()
                    }
                },
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn get_performance_stats(&self) -> Result<Vec<Document>, mongodb::error::Error> {
        self.collection
            .aggregate(
                [
                    doc! {
                        "$group": {
                            "_id": "$backend",
                            "avgTime": { "$avg": "$result.processingTimeMs" },
                            "totalFiles": { "$sum": "$result.totalFiles" },
                            "avgMemory": { "$avg": "$result.memoryUsage" }
                        }
                    },
                    doc! {
                        "$project": {
                            "backend": "$_id",
                            "avgTime": 1,
                            "totalFiles": 1,
                            "avgMemory": 1,
                            "_id": 0
                        }
                    }
                ],
                None,
            )
            .await?
            .try_collect()
            .await
    }
} 