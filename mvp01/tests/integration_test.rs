use anyhow::Result;
use oss_code_analyzer::{Config, run_analysis};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_end_to_end() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = Config {
        input_zip: PathBuf::from("test_data/sample_project.zip"),
        output_dir: temp_dir.path().to_path_buf(),
    };

    run_analysis(config)?;

    // Add assertions to verify the output
    assert!(temp_dir.path().join("LLM-ready-*.json.gz").exists());

    Ok(())
}
