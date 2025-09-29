use std::fs;
use std::path::{Path, PathBuf};

use crate::content_processing::types::*;

/// Content segmentation engine that processes files in 300-line increments with 20-line overlap
/// 
/// # Contract
/// - Processes files in configurable line increments (default 300 lines)
/// - Maintains configurable overlap between chunks (default 20 lines)
/// - Preserves line number information for traceability
/// - Handles edge cases like files smaller than chunk size
/// - Ensures no content is lost between chunks
pub struct ContentSegmenter {
    config: SegmentationConfig,
}

impl ContentSegmenter {
    /// Creates a new content segmenter with the specified configuration
    /// 
    /// # Arguments
    /// * `config` - Segmentation configuration (chunk size, overlap size)
    /// 
    /// # Returns
    /// * `Self` - New content segmenter
    pub fn new(config: SegmentationConfig) -> Self {
        Self { config }
    }
    
    /// Creates a content segmenter with default configuration (300 lines, 20 overlap)
    /// 
    /// # Returns
    /// * `Self` - New content segmenter with default settings
    pub fn with_defaults() -> Self {
        Self::new(SegmentationConfig::default())
    }
    
    /// Segments a file into chunks according to the configuration
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file to segment
    /// 
    /// # Returns
    /// * `Result<Vec<ContentChunk>>` - List of content chunks or error
    /// 
    /// # Contract
    /// - WHEN processing a file THEN it SHALL be divided into chunks of config.chunk_size lines
    /// - WHEN creating overlapping chunks THEN each chunk SHALL include config.overlap_size lines from the previous chunk
    /// - WHEN the file is smaller than chunk_size THEN it SHALL be returned as a single chunk
    /// - WHEN the last chunk would be smaller than overlap_size THEN it SHALL be merged with the previous chunk
    pub fn segment_file(&self, file_path: impl AsRef<Path>) -> Result<Vec<ContentChunk>> {
        let file_path = file_path.as_ref();
        
        if !file_path.exists() {
            return Err(ContentProcessingError::FileNotFound { 
                path: file_path.to_path_buf() 
            });
        }
        
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.is_empty() {
            return Ok(vec![]);
        }
        
        self.segment_lines(file_path.to_path_buf(), &lines)
    }
    
    /// Segments content that's already been read into memory
    /// 
    /// # Arguments
    /// * `source_file` - Path of the source file (for metadata)
    /// * `content` - Content to segment
    /// 
    /// # Returns
    /// * `Result<Vec<ContentChunk>>` - List of content chunks or error
    pub fn segment_content(&self, source_file: PathBuf, content: &str) -> Result<Vec<ContentChunk>> {
        let lines: Vec<&str> = content.lines().collect();
        self.segment_lines(source_file, &lines)
    }
    
    /// Internal method to segment lines into chunks
    fn segment_lines(&self, source_file: PathBuf, lines: &[&str]) -> Result<Vec<ContentChunk>> {
        let total_lines = lines.len();
        
        // If file is smaller than chunk size, return as single chunk
        if total_lines <= self.config.chunk_size {
            let content = lines.join("\n");
            let chunk = ContentChunk::new(
                source_file,
                LineRange::new(1, total_lines),
                content,
                None, // No overlap at start
                None, // No overlap at end
            );
            return Ok(vec![chunk]);
        }
        
        let mut chunks = Vec::new();
        let mut current_line = 0;
        
        while current_line < total_lines {
            let chunk_start = current_line;
            let chunk_end = std::cmp::min(current_line + self.config.chunk_size, total_lines);
            
            // Determine overlap regions
            let overlap_start = if chunk_start > 0 {
                // Include overlap from previous chunk
                let overlap_start_line = chunk_start.saturating_sub(self.config.overlap_size);
                Some(LineRange::new(overlap_start_line + 1, chunk_start))
            } else {
                None
            };
            
            let overlap_end = if chunk_end < total_lines {
                // Include overlap for next chunk
                let overlap_end_line = std::cmp::min(chunk_end + self.config.overlap_size, total_lines);
                Some(LineRange::new(chunk_end + 1, overlap_end_line))
            } else {
                None
            };
            
            // Extract content for this chunk including overlaps
            let content_start = overlap_start.as_ref()
                .map(|r| r.start.saturating_sub(1))
                .unwrap_or(chunk_start);
            let content_end = overlap_end.as_ref()
                .map(|r| r.end)
                .unwrap_or(chunk_end);
            
            let chunk_lines = &lines[content_start..content_end];
            let content = chunk_lines.join("\n");
            
            let chunk = ContentChunk::new(
                source_file.clone(),
                LineRange::new(chunk_start + 1, chunk_end), // 1-based line numbers
                content,
                overlap_start,
                overlap_end,
            );
            
            chunks.push(chunk);
            
            // Move to next chunk, accounting for overlap
            current_line = chunk_end;
            
            // If the remaining content is smaller than overlap size, merge it with the last chunk
            if total_lines - current_line <= self.config.overlap_size && !chunks.is_empty() {
                let remaining_lines = &lines[current_line..];
                if !remaining_lines.is_empty() {
                    let last_chunk = chunks.last_mut().unwrap();
                    
                    // Extend the last chunk to include remaining content
                    let extended_content = format!("{}\n{}", last_chunk.content, remaining_lines.join("\n"));
                    last_chunk.content = extended_content;
                    last_chunk.line_range.end = total_lines;
                    last_chunk.overlap_end = None; // No more overlap needed
                }
                break;
            }
        }
        
        Ok(chunks)
    }
    
    /// Validates that chunks provide complete coverage of the source content
    /// 
    /// # Arguments
    /// * `chunks` - List of chunks to validate
    /// * `total_lines` - Total number of lines in the source file
    /// 
    /// # Returns
    /// * `Result<()>` - Success if coverage is complete, error otherwise
    pub fn validate_coverage(&self, chunks: &[ContentChunk], total_lines: usize) -> Result<()> {
        if chunks.is_empty() {
            if total_lines == 0 {
                return Ok(());
            } else {
                return Err(ContentProcessingError::InvalidLineRange { 
                    start: 0, 
                    end: total_lines 
                });
            }
        }
        
        // Check that first chunk starts at line 1
        if chunks[0].line_range.start != 1 {
            return Err(ContentProcessingError::InvalidLineRange { 
                start: chunks[0].line_range.start, 
                end: chunks[0].line_range.end 
            });
        }
        
        // Check that last chunk ends at total_lines
        if chunks.last().unwrap().line_range.end != total_lines {
            return Err(ContentProcessingError::InvalidLineRange { 
                start: chunks.last().unwrap().line_range.start, 
                end: chunks.last().unwrap().line_range.end 
            });
        }
        
        // Check that chunks are contiguous (allowing for overlap)
        for i in 1..chunks.len() {
            let prev_chunk = &chunks[i - 1];
            let curr_chunk = &chunks[i];
            
            // Current chunk should start where previous chunk ended (or with overlap)
            let expected_start = prev_chunk.line_range.end + 1;
            let actual_start = curr_chunk.line_range.start;
            
            // Allow for overlap
            if actual_start > expected_start {
                return Err(ContentProcessingError::InvalidLineRange { 
                    start: actual_start, 
                    end: curr_chunk.line_range.end 
                });
            }
        }
        
        Ok(())
    }
    
    /// Gets the configuration used by this segmenter
    /// 
    /// # Returns
    /// * `&SegmentationConfig` - Reference to the configuration
    pub fn config(&self) -> &SegmentationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        file
    }
    
    #[test]
    fn test_segment_small_file() {
        let content = (1..=50).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        let file = create_test_file(&content);
        
        let segmenter = ContentSegmenter::with_defaults();
        let chunks = segmenter.segment_file(file.path()).unwrap();
        
        // File is smaller than 300 lines, should be single chunk
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].line_range.start, 1);
        assert_eq!(chunks[0].line_range.end, 50);
        assert!(chunks[0].overlap_start.is_none());
        assert!(chunks[0].overlap_end.is_none());
    }
    
    #[test]
    fn test_segment_large_file() {
        let content = (1..=1000).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        let file = create_test_file(&content);
        
        let segmenter = ContentSegmenter::with_defaults();
        let chunks = segmenter.segment_file(file.path()).unwrap();
        
        // Should have multiple chunks
        assert!(chunks.len() > 1);
        
        // First chunk should start at line 1
        assert_eq!(chunks[0].line_range.start, 1);
        assert_eq!(chunks[0].line_range.end, 300);
        assert!(chunks[0].overlap_start.is_none());
        assert!(chunks[0].overlap_end.is_some());
        
        // Second chunk should have overlap
        assert_eq!(chunks[1].line_range.start, 301);
        assert_eq!(chunks[1].line_range.end, 600);
        assert!(chunks[1].overlap_start.is_some());
        assert!(chunks[1].overlap_end.is_some());
        
        // Validate complete coverage
        segmenter.validate_coverage(&chunks, 1000).unwrap();
    }
    
    #[test]
    fn test_segment_with_custom_config() {
        let content = (1..=500).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        let file = create_test_file(&content);
        
        let config = SegmentationConfig {
            chunk_size: 100,
            overlap_size: 10,
        };
        let segmenter = ContentSegmenter::new(config);
        let chunks = segmenter.segment_file(file.path()).unwrap();
        
        // Should have 5 chunks of 100 lines each
        assert_eq!(chunks.len(), 5);
        
        // Check first chunk
        assert_eq!(chunks[0].line_range.start, 1);
        assert_eq!(chunks[0].line_range.end, 100);
        
        // Check second chunk has overlap
        assert_eq!(chunks[1].line_range.start, 101);
        assert_eq!(chunks[1].line_range.end, 200);
        assert!(chunks[1].overlap_start.is_some());
        
        // Validate coverage
        segmenter.validate_coverage(&chunks, 500).unwrap();
    }
    
    #[test]
    fn test_segment_content_directly() {
        let content = (1..=150).map(|i| format!("Line {}", i)).collect::<Vec<_>>().join("\n");
        
        let segmenter = ContentSegmenter::with_defaults();
        let chunks = segmenter.segment_content(PathBuf::from("test.md"), &content).unwrap();
        
        // Should be single chunk since content is less than 300 lines
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].line_range.start, 1);
        assert_eq!(chunks[0].line_range.end, 150);
    }
    
    #[test]
    fn test_empty_file() {
        let file = create_test_file("");
        
        let segmenter = ContentSegmenter::with_defaults();
        let chunks = segmenter.segment_file(file.path()).unwrap();
        
        assert!(chunks.is_empty());
    }
    
    #[test]
    fn test_file_not_found() {
        let segmenter = ContentSegmenter::with_defaults();
        let result = segmenter.segment_file("nonexistent.txt");
        
        assert!(matches!(result, Err(ContentProcessingError::FileNotFound { .. })));
    }
    
    #[test]
    fn test_validate_coverage() {
        let segmenter = ContentSegmenter::with_defaults();
        
        // Valid coverage
        let chunks = vec![
            ContentChunk::new(
                PathBuf::from("test.md"),
                LineRange::new(1, 300),
                "content1".to_string(),
                None,
                Some(LineRange::new(281, 300)),
            ),
            ContentChunk::new(
                PathBuf::from("test.md"),
                LineRange::new(301, 500),
                "content2".to_string(),
                Some(LineRange::new(281, 300)),
                None,
            ),
        ];
        
        segmenter.validate_coverage(&chunks, 500).unwrap();
        
        // Invalid coverage - gap between chunks
        let invalid_chunks = vec![
            ContentChunk::new(
                PathBuf::from("test.md"),
                LineRange::new(1, 300),
                "content1".to_string(),
                None,
                None,
            ),
            ContentChunk::new(
                PathBuf::from("test.md"),
                LineRange::new(350, 500), // Gap from 301-349
                "content2".to_string(),
                None,
                None,
            ),
        ];
        
        assert!(segmenter.validate_coverage(&invalid_chunks, 500).is_err());
    }
}