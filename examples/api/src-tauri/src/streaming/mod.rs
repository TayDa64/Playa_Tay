// M1 Streaming Hub - Backend Module
// Phase 1 (Foundation): Module structure and exports
// Security: Zero-trust model, all operations validated

pub mod commands;
pub mod database;
pub mod oauth;
pub mod recommendations;

// Re-export types for convenience
pub use commands::*;
pub use database::{
    Recommendation, StreamMetadata, WatchHistoryEntry, WatchQueueEntry,
};
