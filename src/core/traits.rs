//! Core traits for system data components.
//!
//! This module defines the fundamental traits for semantic and geometry data
//! that can be combined into complete system implementations.

use crate::core::state_manager::{Index, IndexPair, Coordinates};

/// Generic systems interface combining semantics and geometry
/// This is the main trait for system implementations
pub trait SystemData {
    // SEMANTIC DATA methods
    fn system_name(&self) -> &'static str;
    fn coherence_attribute(&self) -> &'static str;
    fn term_designation(&self) -> &'static str;
    fn term_characters(&self) -> &[&'static str];
    fn connective_designation(&self) -> &'static str;
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)];
    fn source_attributions(&self) -> &[&'static str];
    
    // GEOMETRY & TOPOLOGY DATA methods
    fn indexes(&self) -> &[Index] { &[] }
    fn coordinates(&self) -> &[Coordinates] { &[] }
    fn edges(&self) -> &[IndexPair] { &[] }
} 

/// Trait for pure semantic data
pub trait SemanticData {
    fn system_name(&self) -> &'static str;
    fn coherence_attribute(&self) -> &'static str;
    fn term_designation(&self) -> &'static str;
    fn term_characters(&self) -> &[&'static str];
    fn connective_designation(&self) -> &'static str;
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)];
    fn source_attributions(&self) -> &[&'static str];
}

/// Trait for pure geometry data
pub trait GeometryData {
    fn indexes(&self) -> &[Index];
    fn coordinates(&self) -> &[Coordinates];
    fn edges(&self) -> &[IndexPair];
}