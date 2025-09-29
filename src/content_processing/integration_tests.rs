use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use crate::content_processing::*;

/// Integration test for the complete content processing infrastructure
/// 
/// Tests the full workflow:
/// 1. Create processing session with directory structure
/// 2. Segment content files into 300-line chunks with 20-line overlap
/// 3. Track progress through all chunks
/// 4. Verify complete coverage and traceability
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_complete_content_processing_workflow() {
        // Create temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().to_path_buf();
        
        // Create test content files (simulating DeepThink Advisory notes)
        let test_files = create_test_advisory_files(&base_dir);
        
        // Step 1: Create processing session manager
        let session_manager = ProcessingSessionManager::new(&base_dir).unwrap();
        
        // Step 2: Create a new processing session
        let session = session_manager.create_session(
            "test_advisory_processing".to_string(),
            test_files.clone(),
            None, // Use default config (300 lines, 20 overlap)
        ).unwrap();
        
        // Verify session directory structure was created
        verify_session_directory_structure(&session);
        
        // Step 3: Initialize progress tracker
        let mut progress_tracker = ProgressTracker::new(&session).unwrap();
        
        // Step 4: Initialize content segmenter
        let segmenter = ContentSegmenter::new(session.config.clone());
        
        // Step 5: Process each source file
        let mut all_chunks = Vec::new();
        
        for source_file in &session.source_files {
            let file_path = base_dir.join(source_file);
            
            // Segment the file into chunks
            let chunks = segmenter.segment_file(&file_path).unwrap();
            
            // Register each chunk with progress tracker
            for chunk in &chunks {
                progress_tracker.register_chunk(chunk).unwrap();
            }
            
            all_chunks.extend(chunks);
        }
        
        // Step 6: Verify initial progress state
        let initial_summary = progress_tracker.get_progress_summary();
        assert_eq!(initial_summary.total_chunks, all_chunks.len());
        assert_eq!(initial_summary.pending_chunks, all_chunks.len());
        assert_eq!(initial_summary.completed_chunks, 0);
        assert_eq!(initial_summary.completion_percentage, 0.0);
        
        // Step 7: Simulate processing chunks
        for chunk in &all_chunks {
            // Mark as processing
            progress_tracker.update_chunk_status(chunk.id, ChunkStatus::Processing).unwrap();
            
            // Simulate processing work
            simulate_chunk_processing(chunk);
            
            // Mark as completed
            progress_tracker.update_chunk_status(chunk.id, ChunkStatus::Completed).unwrap();
        }
        
        // Step 8: Verify final progress state
        let final_summary = progress_tracker.get_progress_summary();
        assert_eq!(final_summary.total_chunks, all_chunks.len());
        assert_eq!(final_summary.completed_chunks, all_chunks.len());
        assert_eq!(final_summary.pending_chunks, 0);
        assert_eq!(final_summary.completion_percentage, 100.0);
        
        // Step 9: Verify content coverage and traceability
        verify_content_coverage(&segmenter, &all_chunks, &test_files, &base_dir);
        
        // Step 10: Test session persistence and recovery
        test_session_persistence(&session_manager, &session);
        
        println!("✅ Complete content processing workflow test passed!");
        println!("   - Processed {} files", test_files.len());
        println!("   - Generated {} chunks", all_chunks.len());
        println!("   - Verified complete coverage and traceability");
    }
    
    fn create_test_advisory_files(base_dir: &PathBuf) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        // Create DTNote01.md (small file - single chunk)
        let note01_content = (1..=150)
            .map(|i| format!("DTNote01 Line {}: Strategic insight about parseltongue architecture", i))
            .collect::<Vec<_>>()
            .join("\n");
        
        let note01_path = base_dir.join("DTNote01.md");
        fs::write(&note01_path, note01_content).unwrap();
        files.push(PathBuf::from("DTNote01.md"));
        
        // Create DTNote02.md (large file - multiple chunks)
        let note02_content = (1..=800)
            .map(|i| format!("DTNote02 Line {}: Technical implementation details for workflow {}", i, i % 10))
            .collect::<Vec<_>>()
            .join("\n");
        
        let note02_path = base_dir.join("DTNote02.md");
        fs::write(&note02_path, note02_content).unwrap();
        files.push(PathBuf::from("DTNote02.md"));
        
        // Create DTNotes03.md (medium file - 2-3 chunks)
        let note03_content = (1..=450)
            .map(|i| format!("DTNotes03 Line {}: User journey analysis for persona type {}", i, i % 5))
            .collect::<Vec<_>>()
            .join("\n");
        
        let note03_path = base_dir.join("DTNotes03.md");
        fs::write(&note03_path, note03_content).unwrap();
        files.push(PathBuf::from("DTNotes03.md"));
        
        files
    }
    
    fn verify_session_directory_structure(session: &ProcessingSession) {
        let base_dir = &session.output_directory;
        
        // Verify all required directories exist
        let required_dirs = [
            "chunks",
            "progress",
            "analysis", 
            "insights",
            "insights/user_journeys",
            "insights/technical",
            "insights/strategic",
            "synthesis",
            "output",
        ];
        
        for dir in &required_dirs {
            let dir_path = base_dir.join(dir);
            assert!(dir_path.exists(), "Directory {} should exist", dir);
            assert!(dir_path.is_dir(), "Path {} should be a directory", dir);
        }
        
        // Verify session metadata file exists
        let metadata_file = base_dir.join("session_metadata.json");
        assert!(metadata_file.exists(), "Session metadata file should exist");
    }
    
    fn simulate_chunk_processing(chunk: &ContentChunk) {
        // Simulate processing by verifying chunk content structure
        assert!(!chunk.content.is_empty(), "Chunk content should not be empty");
        assert!(chunk.line_range.start > 0, "Line range should start from 1 or higher");
        assert!(chunk.line_range.end >= chunk.line_range.start, "Line range should be valid");
        
        // Verify overlap information is consistent
        if let Some(overlap_start) = &chunk.overlap_start {
            assert!(overlap_start.end <= chunk.line_range.start, "Overlap start should be before chunk start");
        }
        
        if let Some(overlap_end) = &chunk.overlap_end {
            assert!(overlap_end.start >= chunk.line_range.end, "Overlap end should be after chunk end");
        }
    }
    
    fn verify_content_coverage(
        segmenter: &ContentSegmenter,
        all_chunks: &[ContentChunk],
        test_files: &[PathBuf],
        base_dir: &PathBuf,
    ) {
        // Group chunks by source file
        let mut chunks_by_file = std::collections::HashMap::new();
        for chunk in all_chunks {
            chunks_by_file
                .entry(chunk.source_file.clone())
                .or_insert_with(Vec::new)
                .push(chunk);
        }
        
        // Verify coverage for each file
        for file_path in test_files {
            let full_path = base_dir.join(file_path);
            let content = fs::read_to_string(&full_path).unwrap();
            let total_lines = content.lines().count();
            
            let file_chunks = chunks_by_file.get(file_path)
                .or_else(|| chunks_by_file.get(&full_path))
                .unwrap_or_else(|| {
                    println!("Available keys in chunks_by_file:");
                    for key in chunks_by_file.keys() {
                        println!("  {:?}", key);
                    }
                    println!("Looking for: {:?}", file_path);
                    panic!("Could not find chunks for file");
                });
            
            // Convert Vec<&ContentChunk> to Vec<ContentChunk> for validation
            let owned_chunks: Vec<ContentChunk> = file_chunks.iter().map(|&chunk| chunk.clone()).collect();
            
            // Verify coverage using segmenter's validation
            segmenter.validate_coverage(&owned_chunks, total_lines).unwrap();
            
            // Verify chunk content matches source
            verify_chunk_content_matches_source(file_chunks, &content);
        }
    }
    
    fn verify_chunk_content_matches_source(chunks: &[&ContentChunk], source_content: &str) {
        let source_lines: Vec<&str> = source_content.lines().collect();
        
        for chunk in chunks {
            let chunk_lines: Vec<&str> = chunk.content.lines().collect();
            
            // For the main content (excluding overlaps), verify it matches source
            let start_idx = chunk.line_range.start - 1; // Convert to 0-based
            let end_idx = chunk.line_range.end;
            
            let expected_main_lines = &source_lines[start_idx..end_idx];
            
            // The chunk content should contain at least the main lines
            // (it may have additional overlap content)
            for (i, expected_line) in expected_main_lines.iter().enumerate() {
                let chunk_line_idx = if chunk.overlap_start.is_some() {
                    // Account for overlap at the beginning
                    i + chunk.overlap_start.as_ref().unwrap().len()
                } else {
                    i
                };
                
                if chunk_line_idx < chunk_lines.len() {
                    assert!(
                        chunk_lines[chunk_line_idx].contains(&expected_line[..std::cmp::min(20, expected_line.len())]),
                        "Chunk content should match source content"
                    );
                }
            }
        }
    }
    
    fn test_session_persistence(
        session_manager: &ProcessingSessionManager,
        original_session: &ProcessingSession,
    ) {
        // Test loading the session
        let loaded_session = session_manager.load_session(&original_session.name).unwrap();
        
        assert_eq!(loaded_session.id, original_session.id);
        assert_eq!(loaded_session.name, original_session.name);
        assert_eq!(loaded_session.source_files, original_session.source_files);
        assert_eq!(loaded_session.config.chunk_size, original_session.config.chunk_size);
        assert_eq!(loaded_session.config.overlap_size, original_session.config.overlap_size);
        
        // Test listing sessions
        let sessions = session_manager.list_sessions().unwrap();
        assert!(sessions.contains(&original_session.name));
    }
}