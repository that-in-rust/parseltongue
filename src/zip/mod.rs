// Level 4: ZIP Module
// - Manages ZIP file processing
// - Coordinates streaming and processing
// - Handles compression and encoding

pub mod codec;
pub mod entry_processor;
pub mod zip_processor;

// Re-export commonly used types
pub use codec::ZipEntryCodec;
pub use zip_processor::process_zip;