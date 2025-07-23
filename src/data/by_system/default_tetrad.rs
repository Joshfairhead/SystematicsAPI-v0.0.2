use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the tetrad system
#[derive(Debug, Clone)]
pub struct DefaultTetradSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 4],
    /// Relationships between terms (6 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 6],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 4],
    pub coordinates: [Coordinates; 4],
    pub edges: [(usize, usize); 6],
}

impl Default for DefaultTetradSystem {
    fn default() -> Self {
        Self {
            name: "Tetrad",
            coherence_attribute: "Activity Field",
            term_designation: "Sources",
            term_characters: ["Ideal", "Directive", "Instrumental", "Ground"],
            connective_designation: "Interplays",
            connective_characters: [
                ("Receptive Regard", "Ideal", "Directive"),
                ("Effectual Compatibility", "Ideal", "Instrumental"),
                ("Motivational Imperative", "Ideal", "Ground"),
                ("Demonstrable Activity", "Directive", "Instrumental"),
                ("Material Mastery", "Directive", "Ground"),
                ("Technical Power", "Instrumental", "Ground"),
            ],
            source_attributions: ["Elementary Systematics", "Bennett's Work"],
            
            // Geometry data from archive
            indexes: [0, 1, 2, 3],
            coordinates: [
                Coordinates { x: 0.0, y: 1.0, z: None },   // ideal
                Coordinates { x: 1.0, y: 0.0, z: None },   // directive
                Coordinates { x: -1.0, y: 0.0, z: None },  // instrumental
                Coordinates { x: 0.0, y: -1.0, z: None },  // ground
            ],
            edges: [
                (0, 1),  // ideal-directive
                (0, 2),  // ideal-instrumental
                (0, 3),  // ideal-ground
                (1, 2),  // directive-instrumental
                (1, 3),  // directive-ground
                (2, 3),  // instrumental-ground
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultTetradSystem {
    fn system_name(&self) -> &'static str {
        self.name
    }
    fn coherence_attribute(&self) -> &'static str {
        self.coherence_attribute
    }
    fn term_designation(&self) -> &'static str {
        self.term_designation
    }
    fn term_characters(&self) -> &[&'static str] {
        &self.term_characters
    }
    fn connective_designation(&self) -> &'static str {
        self.connective_designation
    }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] {
        &self.connective_characters
    }
    fn source_attributions(&self) -> &[&'static str] {
        &self.source_attributions
    }
    
    // Geometry methods
    fn indexes(&self) -> &[usize] {
        &self.indexes
    }
    fn coordinates(&self) -> &[Coordinates] {
        &self.coordinates
    }
    fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}

