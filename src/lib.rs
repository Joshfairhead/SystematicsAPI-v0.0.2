//! # systematic-constructor
//!
//! This crate provides a stable implementation of a systematic constructor, extracted from a larger project for reuse and further development.
//!
//! Main modules:
//! - core: Central types, traits, and state management
//! - data: System-specific data configurations

pub mod core;
pub mod data;

// Re-export commonly used items
pub use core::term_characters::Term; 