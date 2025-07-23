use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Default triad system configuration
#[derive(Debug, Clone)]
pub struct DefaultTriadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 3],
    /// Relationships between terms (3 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 3],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 3],
    pub coordinates: [Coordinates; 3],
    pub edges: [(usize, usize); 3],
}

impl Default for DefaultTriadSystem {
    fn default() -> Self {
        Self {
            name: "Triad",
            coherence_attribute: "Dynamism",
            term_designation: "Impulses",
            connective_designation: "Acts",
            term_characters: ["Will", "Function", "Being"],
            connective_characters: [
                ("Act1", "Will", "Function"),
                ("Act2", "Function", "Being"),
                ("Act3", "Being", "Will"),
            ],
            source_attributions: ["Elementary Systematics", "Bennett's Work"],
            
            // Geometry data from archive
            indexes: [0, 1, 2],
            coordinates: [
                Coordinates { x: 0.0, y: 1.0, z: None },   // Will (top left)
                Coordinates { x: 0.0, y: -1.0, z: None },  // Function (bottom left)
                Coordinates { x: 1.0, y: 0.0, z: None },   // Being (right, midpoint vertically)
            ],
            edges: [
                (0, 1),  // Will-Function (vertical left)
                (1, 2),  // Function-Being (bottom right)
                (2, 0),  // Being-Will (top right)
            ],
        }
    }
}

/// Access the data from the default system struct
impl SystemData for DefaultTriadSystem {
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