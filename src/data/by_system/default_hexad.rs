use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the hexad system
#[derive(Debug, Clone)]
pub struct DefaultHexadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 6],
    /// Relationships between terms (15 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 15],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 6],
    pub coordinates: [Coordinates; 6],
    pub edges: [(usize, usize); 15],
}

impl Default for DefaultHexadSystem {
    fn default() -> Self {
        Self {
            name: "Hexad",
            coherence_attribute: "Coalescence",
            term_designation: "Laws",
            term_characters: ["Resources", "Values", "Options", "Criteria", "Facts", "Priorities"],
            connective_designation: "Steps",
            connective_characters: [
                ("Step1", "Resources", "Values"),
                ("Step2", "Resources", "Options"),
                ("Step3", "Resources", "Criteria"),
                ("Step4", "Resources", "Facts"),
                ("Step5", "Resources", "Priorities"),
                ("Step6", "Values", "Options"),
                ("Step7", "Values", "Criteria"),
                ("Step8", "Values", "Facts"),
                ("Step9", "Values", "Priorities"),
                ("Step10", "Options", "Criteria"),
                ("Step11", "Options", "Facts"),
                ("Step12", "Options", "Priorities"),
                ("Step13", "Criteria", "Facts"),
                ("Step14", "Criteria", "Priorities"),
                ("Step15", "Facts", "Priorities"),
            ],
            source_attributions: ["Elementary Systematics", "Bennett's Work"],
            
            // Geometry data from archive
            indexes: [0, 1, 2, 3, 4, 5],
            coordinates: [
                Coordinates { x: -0.5, y: 0.86602540378, z: None }, // index 0: top left
                Coordinates { x: 0.0, y: 1.0, z: None },            // index 1: top tip
                Coordinates { x: 0.5, y: 0.86602540378, z: None },  // index 2: top right
                Coordinates { x: 1.0, y: 0.0, z: None },            // index 3: bottom right
                Coordinates { x: 0.0, y: -1.0, z: None },           // index 4: bottom
                Coordinates { x: -1.0, y: 0.0, z: None },           // index 5: bottom left
            ],
            edges: [
                (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
                (1, 2), (1, 3), (1, 4), (1, 5),
                (2, 3), (2, 4), (2, 5),
                (3, 4), (3, 5),
                (4, 5),
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultHexadSystem {
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

    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] {
        &self.connective_characters
    }
    fn connective_designation(&self) -> &'static str {
        self.connective_designation
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
