//! Core Infrastructure Layer
//! 
//! Pyramid Structure:
//! 
//! Level 4 (Top): Core Coordination
//! - Core API         (public interface)
//! - Error Handling   (error propagation)
//! - Type System      (type coordination)
//! 
//! Level 3: Module Management
//! - Channel System   (async channels)
//! - Runtime System   (async runtime)
//! - Type System      (core types)
//! 
//! Level 2: Error Management
//! - Error Types      (error definitions)
//! - Error Handling   (error processing)
//! 
//! Level 1 (Base): Module Exports
//! - Public Exports   (module visibility)
//! - Type Exports     (type visibility)

pub mod channel;
pub mod error;
pub mod types;
pub mod runtime;
