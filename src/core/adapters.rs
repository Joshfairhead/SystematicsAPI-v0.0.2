use crate::core::traits::{SystemData, GeometryData, SemanticData};
use crate::core::state_manager::{Index, IndexPair, Coordinates};

/// SystemDataAdapter struct that combines semantic data with geometry data
/// This implements SystemData to provide a complete system interface
pub struct SystemDataAdapter<S, G> {
    semantics: S,
    geometry: G,
}

impl<S, G> SystemDataAdapter<S, G> {
    pub fn new(semantics: S, geometry: G) -> Self {
        Self { semantics, geometry }
    }
}

// Implement SystemData for SystemDataAdapter (complete unified interface)
impl<S, G> SystemData for SystemDataAdapter<S, G>
where
    S: SemanticData,
    G: GeometryData,
{
    // Semantic methods
    fn system_name(&self) -> &'static str {
        self.semantics.system_name()
    }
    
    fn coherence_attribute(&self) -> &'static str {
        self.semantics.coherence_attribute()
    }
    
    fn term_designation(&self) -> &'static str {
        self.semantics.term_designation()
    }
    
    fn term_characters(&self) -> &[&'static str] {
        self.semantics.term_characters()
    }
    
    fn connective_designation(&self) -> &'static str {
        self.semantics.connective_designation()
    }
    
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] {
        self.semantics.connective_characters()
    }
    
    fn source_attributions(&self) -> &[&'static str] {
        self.semantics.source_attributions()
    }
    
    // Geometry & topology methods
    fn indexes(&self) -> &[Index] {
        self.geometry.indexes()
    }
    
    fn coordinates(&self) -> &[Coordinates] {
        self.geometry.coordinates()
    }
    
    fn edges(&self) -> &[IndexPair] {
        self.geometry.edges()
    }
}

// Implement GeometryData for SystemDataAdapter (for when we need pure geometry access)
impl<S, G> GeometryData for SystemDataAdapter<S, G>
where
    S: SemanticData,
    G: GeometryData,
{
    fn indexes(&self) -> &[Index] {
        self.geometry.indexes()
    }
    
    fn coordinates(&self) -> &[Coordinates] {
        self.geometry.coordinates()
    }
    
    fn edges(&self) -> &[IndexPair] {
        self.geometry.edges()
    }
}

// Implement SemanticData for SystemDataAdapter (for when we need pure semantic access)
impl<S, G> SemanticData for SystemDataAdapter<S, G>
where
    S: SemanticData,
    G: GeometryData,
{
    fn system_name(&self) -> &'static str {
        self.semantics.system_name()
    }
    
    fn coherence_attribute(&self) -> &'static str {
        self.semantics.coherence_attribute()
    }
    
    fn term_designation(&self) -> &'static str {
        self.semantics.term_designation()
    }
    
    fn term_characters(&self) -> &[&'static str] {
        self.semantics.term_characters()
    }
    
    fn connective_designation(&self) -> &'static str {
        self.semantics.connective_designation()
    }
    
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] {
        self.semantics.connective_characters()
    }
    
    fn source_attributions(&self) -> &[&'static str] {
        self.semantics.source_attributions()
    }
} 