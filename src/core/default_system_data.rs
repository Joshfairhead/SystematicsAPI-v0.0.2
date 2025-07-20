//! Default system data trait for loading standard system configurations.
//!
//! This trait defines the interface that any default system data must implement
//! to be loadable into the state manager.

use crate::core::state_manager::{Index, IndexPair, Coordinates};

/// Trait for loading default data from any system configuration
pub trait DefaultSystemData {
    /// The name of the system
    fn system_name(&self) -> &'static str;
    
    /// The primary organizing principle (coherence attribute)
    fn coherence_attribute(&self) -> &'static str;

    /// The designation for terms (e.g., "Elements", "Impulses")
    fn term_designation(&self) -> &'static str;
    
    /// The ordered list of term characters for each position
    fn term_characters(&self) -> &[&'static str];
    
    /// The designation for connectives (e.g., "Components", "Acts")
    fn connective_designation(&self) -> &'static str;
    
    /// The relationships between terms as (connective_name, term1, term2) tuples
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)];
    
    /// The source attributions for this system
    fn source_attributions(&self) -> &[&'static str];

    // Geometry methods (optional - return empty arrays for systems without geometry)
    /// The indexes for this system (defaults to empty for systems without geometry)
    fn indexes(&self) -> &[Index] { &[] }
    
    /// The coordinates for each index (defaults to empty for systems without geometry)
    fn coordinates(&self) -> &[Coordinates] { &[] }
    
    /// The edges (index pairs) for this system (defaults to empty for systems without geometry)
    fn edges(&self) -> &[IndexPair] { &[] }
} 