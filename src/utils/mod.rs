// Level 4: Utilities Module
// - Coordinates utility components
// - Manages shared resources
// - Handles initialization
// - Provides metrics points

pub mod buffer;
pub mod cleanup;
pub mod resource;

// Level 3: Initialization
pub fn init() {
    buffer::init();
    resource::init();
    cleanup::init();
}

// Level 2: Resource Management
pub use buffer::BufferPool;
pub use resource::ResourcePool;
pub use cleanup::CleanupManager;

// Level 1: Type Exports
pub type Buffer = bytes::BytesMut;
pub type Resource<T> = std::sync::Arc<T>; 