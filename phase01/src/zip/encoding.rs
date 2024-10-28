//! ZIP Entry Encoding Management
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Encoding Management
//! - EncodingManager   (manages encodings)
//! - EncodingMetrics   (tracks encoding usage)
//! - FallbackManager   (manages fallbacks)
//! 
//! Level 3: Encoding Operations
//! - EncodingDetector  (detects encodings)
//! - EncodingConverter (converts encodings)
//! - FallbackHandler   (handles failures)
//! 
//! Level 2: Encoding Implementation
//! - AsyncEncoder      (async encoding)
//! - EncodingState     (encoding state)
//! - ConversionBuffer  (conversion buffer)
//! 
//! Level 1 (Base): Core Encoding Types
//! - EncodingConfig    (encoding config)
//! - EncodingResult    (result types)
//! - EncodingError     (encoding errors)

use std::sync::Arc;
use encoding_rs::{Encoding, UTF_8, WINDOWS_1252, CoderResult};
use chardetng::EncodingDetector as ChardetDetector;
use metrics::{Counter, Gauge};
use crate::core::{error::{Error, Result}, types::*};

// ===== Level 1: Core Encoding Types =====
// Design Choice: Using encoding_rs for reliable conversion

/// Encoding metrics collection
#[derive(Debug, Default)]
struct EncodingMetrics {
    successful_conversions: Counter,
    conversion_errors: Counter,
    fallbacks_used: Counter,
    active_conversions: Gauge,
}

impl EncodingMetrics {
    fn new() -> Self {
        Self::default()
    }
}

/// Encoding configuration
#[derive(Debug, Clone)]
pub struct EncodingConfig {
    /// Default encoding
    pub default_encoding: &'static Encoding,
    /// Fallback encoding
    pub fallback_encoding: &'static Encoding,
    /// Enable detection
    pub enable_detection: bool,
    /// Detection confidence threshold
    pub detection_threshold: f32,
}

impl Default for EncodingConfig {
    fn default() -> Self {
        Self {
            default_encoding: UTF_8,
            fallback_encoding: WINDOWS_1252,
            enable_detection: true,
            detection_threshold: 0.8,
        }
    }
}

// ===== Level 2: Encoding Implementation =====
// Design Choice: Using detection with fallback

/// Encoding detector implementation
pub struct EncodingDetector {
    /// Encoding configuration
    config: EncodingConfig,
    /// Encoding metrics
    metrics: EncodingMetrics,
}

impl EncodingDetector {
    /// Creates new encoding detector
    pub fn new(config: EncodingConfig) -> Self {
        let metrics = EncodingMetrics::new();

        Self {
            config,
            metrics,
        }
    }

    // ===== Level 3: Encoding Operations =====
    // Design Choice: Using separate functions for detection and conversion

    /// Detects and converts encoding
    pub fn detect_and_convert(&self, input: &[u8]) -> Result<String> {
        self.metrics.active_conversions.increment(1.0);

        let result = if self.config.enable_detection {
            // Try to detect encoding
            if let Some(encoding) = self.detect_encoding(input) {
                self.convert_with_encoding(input, encoding)
            } else {
                // Try default encoding
                self.convert_with_encoding(input, self.config.default_encoding)
            }
        } else {
            // Use default encoding directly
            self.convert_with_encoding(input, self.config.default_encoding)
        };

        self.metrics.active_conversions.decrement(1.0);
        result
    }

    /// Detects encoding from input
    fn detect_encoding(&self, input: &[u8]) -> Option<&'static Encoding> {
        let mut detector = ChardetDetector::new();
        detector.feed(input, true);
        
        let encoding = detector.guess(None, true);
        let confidence = detector.confidence();
        
        if confidence >= self.config.detection_threshold {
            Some(encoding)
        } else {
            None
        }
    }

    /// Converts input with specified encoding
    fn convert_with_encoding(&self, input: &[u8], encoding: &'static Encoding) -> Result<String> {
        let (cow, _encoding_used, had_errors) = encoding.decode(input);
        
        if had_errors {
            // Try fallback encoding if primary fails
            let (fallback_cow, _, fallback_errors) = self.config.fallback_encoding.decode(input);
            
            if fallback_errors {
                self.metrics.conversion_errors.increment(1);
                Err(Error::EncodingFailed("Both primary and fallback encoding failed".into()))
            } else {
                self.metrics.fallbacks_used.increment(1);
                Ok(fallback_cow.into_owned())
            }
        } else {
            self.metrics.successful_conversions.increment(1);
            Ok(cow.into_owned())
        }
    }
}

// ===== Level 4: Encoding Management =====
// Design Choice: Using builder pattern for encoding chain

/// Encoding chain builder
pub struct EncodingChain {
    encodings: Vec<&'static Encoding>,
    threshold: f32,
}

impl EncodingChain {
    pub fn new() -> Self {
        Self {
            encodings: Vec::new(),
            threshold: 0.8,
        }
    }

    pub fn add_encoding(&mut self, encoding: &'static Encoding) -> &mut Self {
        self.encodings.push(encoding);
        self
    }

    pub fn with_threshold(&mut self, threshold: f32) -> &mut Self {
        self.threshold = threshold;
        self
    }

    pub fn convert(&self, input: &[u8]) -> Result<String> {
        for encoding in &self.encodings {
            let (cow, _, had_errors) = encoding.decode(input);
            if !had_errors {
                return Ok(cow.into_owned());
            }
        }
        
        Err(Error::EncodingFailed("All encodings failed".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_conversion() {
        let config = EncodingConfig::default();
        let detector = EncodingDetector::new(config);

        let input = b"Hello, World!";
        let result = detector.detect_and_convert(input);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_windows1252_fallback() {
        let config = EncodingConfig::default();
        let detector = EncodingDetector::new(config);

        // Windows-1252 encoded bytes
        let input = &[0x48, 0xE9, 0x6C, 0x6C, 0x6F]; // "Héllo"
        let result = detector.detect_and_convert(input);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Héllo");
    }

    #[test]
    fn test_encoding_chain() {
        let mut chain = EncodingChain::new();
        chain
            .add_encoding(UTF_8)
            .add_encoding(WINDOWS_1252);

        let input = b"Hello, World!";
        let result = chain.convert(input);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }
}
