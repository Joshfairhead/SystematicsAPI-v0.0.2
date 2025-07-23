use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the pentad system
#[derive(Debug, Clone)]
pub struct DefaultPentadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 5],
    /// Relationships between terms (10 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 10],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 5],
    pub coordinates: [Coordinates; 5],
    pub edges: [(usize, usize); 10],
}

impl Default for DefaultPentadSystem {
    fn default() -> Self {
        Self {
            name: "Pentad",
            coherence_attribute: "Significance and Potential",
            term_designation: "Limits",
            term_characters: ["Purpose", "Higher Potential", "Quintessence", "Lower Potential", "Source"],
            connective_designation: "Mutualities",
            connective_characters: [
                ("Range of Potential", "Higher Potential", "Lower Potential"),
                ("Range of Significance", "Purpose", "Source"),
                ("Aspiration", "Quintessence", "Higher Potential"),
                ("Operation", "Quintessence", "Lower Potential"),
                ("Output", "Higher Potential", "Purpose"),
                ("Input", "Lower Potential", "Source"),
                ("Qualitative Match", "Quintessence", "Purpose"),
                ("Quantitative Match", "Quintessence", "Source"),
                ("Form", "Lower Potential", "Purpose"),
                ("Function", "Higher Potential", "Source"),
            ],
            source_attributions: ["Elementary Systematics", "Bennett's Work"],
            
            // Geometry data from archive
            indexes: [0, 1, 2, 3, 4],
            coordinates: [
                Coordinates { x: 1.0, y: 1.0, z: None },     // 0: Purpose (upper right)
                Coordinates { x: 0.5, y: 0.5, z: None },     // 1: Higher Potential (middle, above quintessence, below purpose)
                Coordinates { x: -1.0, y: 0.0, z: None },    // 2: Quintessence (left, vertical midpoint)
                Coordinates { x: 0.5, y: -0.5, z: None },    // 3: Lower Potential (middle, below quintessence, above source)
                Coordinates { x: 1.0, y: -1.0, z: None },    // 4: Source (lower right)
            ],
            edges: [
                (0, 1), // Purpose-Higher Potential
                (1, 2), // Higher Potential-Quintessence
                (2, 3), // Quintessence-Lower Potential
                (3, 4), // Lower Potential-Source
                (4, 0), // Source-Purpose
                (0, 2), // Purpose-Quintessence
                (1, 3), // Higher Potential-Lower Potential
                (2, 4), // Quintessence-Source
                (3, 0), // Lower Potential-Purpose
                (4, 1), // Source-Higher Potential
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultPentadSystem {
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

