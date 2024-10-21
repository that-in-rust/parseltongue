use anyhow::{Context, Result};
use chrono::Local;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use prost::Message;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::proto::{ProjectSummary as ProtoProjectSummary, FileSummary as ProtoFileSummary};
use crate::ParsedFile;

pub struct OutputManager {
    output_dir: PathBuf,
}

impl OutputManager {
    pub fn new(output_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
        Ok(Self { output_dir })
    }

    pub fn write_llm_ready_output(&self, files: &[ParsedFile]) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let path = self.output_dir.join(format!("LLM-ready-{}.pb.gz", timestamp));
        let file = File::create(path).context("Failed to create LLM-ready output file")?;
        let mut encoder = ZlibEncoder::new(file, Compression::default());

        let proto_files: Vec<ProtoFileSummary> = files.iter().map(|f| ProtoFileSummary {
            name: f.name.clone(),
            language: f.language.to_string(),
            loc: f.loc as u32,
            code: f.code as u32,
            comments: f.comments as u32,
            blanks: f.blanks as u32,
            cyclomatic_complexity: f.cyclomatic_complexity as u32,
            cognitive_complexity: f.cognitive_complexity as u32,
        }).collect();

        let proto_summary = ProtoProjectSummary {
            files: proto_files,
            total_loc: files.iter().map(|f| f.loc).sum::<usize>() as u32,
        };

        proto_summary.encode(&mut encoder).context("Failed to encode and compress summary")?;
        encoder.finish().context("Failed to finish compression")?;

        Ok(())
    }

    pub fn write_progress(&self, message: &str) -> Result<()> {
        let path = self.output_dir.join("processProgress.txt");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .context("Failed to open progress file")?;
        writeln!(file, "[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message)
            .context("Failed to write to progress file")?;
        Ok(())
    }

    pub fn cleanup_old_files(&self, max_files: usize) -> Result<()> {
        let mut files: Vec<_> = std::fs::read_dir(&self.output_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("gz") {
                    Some((entry.metadata().ok()?.modified().ok()?, path))
                } else {
                    None
                }
            })
            .collect();

        if files.len() > max_files {
            files.sort_by(|a, b| b.0.cmp(&a.0));
            for (_, path) in files.iter().skip(max_files) {
                remove_file(path).context("Failed to remove old output file")?;
            }
        }

        Ok(())
    }
}
