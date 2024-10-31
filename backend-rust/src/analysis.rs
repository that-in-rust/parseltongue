/**
 * Analysis Engine Pyramid:
 * L1: Core analysis orchestration
 * L2: Git operations
 * L3: File processing
 * L4: Metrics collection
 */

use tokio::fs;
use git2::Repository;
use std::time::Instant;
use mongodb::bson::doc;
use crate::db::AnalysisStore;

pub struct AnalysisEngine {
    store: AnalysisStore,
    cache_dir: String,
    repo_url: String,
    start_time: Option<Instant>,
    files_processed: u64,
}

impl AnalysisEngine {
    pub fn new(store: AnalysisStore, config: &Config) -> Self {
        Self {
            store,
            cache_dir: config.cache_dir.clone(),
            repo_url: config.repo_url.clone(),
            start_time: None,
            files_processed: 0,
        }
    }

    pub async fn start_analysis(&mut self, job_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.start_time = Some(Instant::now());
        self.files_processed = 0;

        // L1: Core orchestration
        self.store.create_job(job_id).await?;
        
        // L2: Git operations
        self.ensure_repo().await?;
        
        // L3: File processing
        self.process_files(job_id).await?;
        
        // L4: Metrics collection
        self.collect_metrics(job_id).await?;
        
        Ok(())
    }

    async fn ensure_repo(&self) -> Result<(), Box<dyn std::error::Error>> {
        let repo_path = format!("{}/repo", self.cache_dir);
        if !fs::metadata(&repo_path).await.is_ok() {
            Repository::clone(&self.repo_url, &repo_path)?;
        }
        Ok(())
    }

    async fn process_files(&mut self, job_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = fs::read_dir(format!("{}/repo", self.cache_dir)).await?;
        let mut language_breakdown = std::collections::HashMap::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_string();
                *language_breakdown.entry(ext).or_insert(0) += 1;
                self.files_processed += 1;
            }

            self.store.update_progress(
                job_id,
                self.files_processed as f64,
                &path.to_string_lossy()
            ).await?;
        }

        Ok(())
    }

    async fn collect_metrics(&self, job_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let duration = self.start_time.unwrap().elapsed();
        let files_per_second = self.files_processed as f64 / duration.as_secs_f64();
        
        self.store.update_metrics(job_id, doc! {
            "processingTimeMs": duration.as_millis() as i64,
            "filesPerSecond": files_per_second,
            "totalFiles": self.files_processed,
            "memoryUsage": get_memory_usage()
        }).await?;

        Ok(())
    }
}

fn get_memory_usage() -> u64 {
    // Platform-specific memory usage implementation
    #[cfg(target_os = "linux")]
    {
        use std::fs::File;
        use std::io::Read;
        let mut status = String::new();
        File::open("/proc/self/status")?.read_to_string(&mut status)?;
        // Parse VmRSS value
        status.lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|kb| kb.parse().ok())
            .unwrap_or(0)
    }
    #[cfg(not(target_os = "linux"))]
    {
        0 // Fallback for non-Linux platforms
    }
} 