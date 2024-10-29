// Level 4: ZIP Module Organization
// - Manages ZIP processing
// - Coordinates streaming
// - Handles compression
// - Provides validation

pub mod stream;
pub mod codec;
pub mod validation;
pub mod compression;
pub mod entry_processor;
pub mod reader;

pub use stream::ZipStream;
pub use codec::ZipCodec;
pub use validation::ZipValidator;