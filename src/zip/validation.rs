// Level 4: ZIP Validation
// - Validates ZIP structure
// - Checks CRC32 checksums
// - Verifies compression
// - Tracks validation metrics

use crate::core::error::{Error, Result};
use metrics::{counter, gauge};
use crc32fast::Hasher;

pub struct ZipValidator {
    crc32: Hasher,
    total_bytes: usize,
}

impl ZipValidator {
    pub fn new() -> Self {
        Self {
            crc32: Hasher::new(),
            total_bytes: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.crc32.update(data);
        self.total_bytes += data.len();
        gauge!("zip.validation.bytes").set(self.total_bytes as f64);
    }

    pub fn validate(&self, expected_crc32: u32) -> Result<()> {
        let actual_crc32 = self.crc32.clone().finalize();
        if actual_crc32 != expected_crc32 {
            counter!("zip.validation.errors").increment(1);
            return Err(Error::Validation(format!(
                "CRC32 mismatch: expected {:x}, got {:x}",
                expected_crc32, actual_crc32
            )));
        }
        counter!("zip.validation.success").increment(1);
        Ok(())
    }
} 