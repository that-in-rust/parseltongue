// Level 4: ZIP Validation
// - Implements CRC32 checking
// - Validates ZIP structure
// - Handles character encodings
// - Reports validation metrics

use crc32fast::Hasher;
use encoding_rs::Encoding;
use crate::core::error::{Error, Result};

// Level 3: Validation Types
pub struct ZipValidator {
    hasher: Hasher,
    encoding: &'static Encoding,
}

impl ZipValidator {
    // Level 2: Validation Operations
    pub fn validate_entry(&mut self, data: &[u8], expected_crc: u32) -> Result<()> {
        self.hasher.update(data);
        let actual_crc = self.hasher.finalize();
        
        if actual_crc != expected_crc {
            return Err(Error::Processing { 
                msg: format!("CRC mismatch: expected {}, got {}", expected_crc, actual_crc) 
            });
        }
        Ok(())
    }

    // Level 1: Encoding Operations
    pub fn decode_filename(&self, raw: &[u8]) -> Result<String> {
        let (cow, _, had_errors) = self.encoding.decode(raw);
        if had_errors {
            return Err(Error::Processing { 
                msg: "Failed to decode filename".into() 
            });
        }
        Ok(cow.into_owned())
    }
} 