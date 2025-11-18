// Library exports for testing and external use

mod agent;

pub mod migrate;

// Re-export commonly used types
pub use migrate::{CISystem, SourceConfig, detect_source_configs};
