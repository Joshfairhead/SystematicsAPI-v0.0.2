use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the heptad system
#[derive(Debug, Clone)]
pub struct DefaultHeptadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 7],
    /// Relationships between terms (21 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 21],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 7],
    pub coordinates: [Coordinates; 7],
    pub edges: [(usize, usize); 21],
}

impl Default for DefaultHeptadSystem {
    fn default() -> Self {
        Self {
            name: "Heptad",
            coherence_attribute: "Generation",
            term_designation: "States",
            term_characters: ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"],
            connective_designation: "Intervals",
            connective_characters: [
                ("Needs Research", "Insight", "Research"),
                ("Needs Research", "Insight", "Design"),
                ("Needs Research", "Insight", "Synthesis"),
                ("Needs Research", "Insight", "Application"),
                ("Needs Research", "Insight", "Delivery"),
                ("Needs Research", "Insight", "Value"),
                ("Needs Research", "Research", "Design"),
                ("Needs Research", "Research", "Synthesis"),
                ("Needs Research", "Research", "Application"),
                ("Needs Research", "Research", "Delivery"),
                ("Needs Research", "Research", "Value"),
                ("Needs Research", "Design", "Synthesis"),
                ("Needs Research", "Design", "Application"),
                ("Needs Research", "Design", "Delivery"),
                ("Needs Research", "Design", "Value"),
                ("Needs Research", "Synthesis", "Application"),
                ("Needs Research", "Synthesis", "Delivery"),
                ("Needs Research", "Synthesis", "Value"),
                ("Needs Research", "Application", "Delivery"),
                ("Needs Research", "Application", "Value"),
                ("Needs Research", "Delivery", "Value"),
            ],
            source_attributions: ["Elementary Systematics", "Bennett's Work"],
            
            // Geometry data from archive
            indexes: [0, 1, 2, 3, 4, 5, 6],
            coordinates: [
                Coordinates { x: 0.0, y: 1.0, z: None },             // 0: Insight (top center)
                Coordinates { x: 0.781831, y: 0.623489, z: None },   // 1: Research
                Coordinates { x: 0.974370, y: -0.222521, z: None },  // 2: Design
                Coordinates { x: 0.433884, y: -0.900969, z: None },  // 3: Synthesis
                Coordinates { x: -0.433884, y: -0.900969, z: None }, // 4: Application
                Coordinates { x: -0.974370, y: -0.222521, z: None }, // 5: Delivery
                Coordinates { x: -0.781831, y: 0.623489, z: None },  // 6: Value
            ],
            edges: [
                (0, 1), // Insight-Research
                (0, 2), // Insight-Design
                (0, 3), // Insight-Synthesis
                (0, 4), // Insight-Application
                (0, 5), // Insight-Delivery
                (0, 6), // Insight-Value
                (1, 2), // Research-Design
                (1, 3), // Research-Synthesis
                (1, 4), // Research-Application
                (1, 5), // Research-Delivery
                (1, 6), // Research-Value
                (2, 3), // Design-Synthesis
                (2, 4), // Design-Application
                (2, 5), // Design-Delivery
                (2, 6), // Design-Value
                (3, 4), // Synthesis-Application
                (3, 5), // Synthesis-Delivery
                (3, 6), // Synthesis-Value
                (4, 5), // Application-Delivery
                (4, 6), // Application-Value
                (5, 6), // Delivery-Value
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultHeptadSystem {
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
