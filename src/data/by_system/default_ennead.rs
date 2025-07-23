use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the ennead system
/// Note: This is a placeholder implementation as the term characters require authentic Bennett research
#[derive(Debug, Clone)]
pub struct DefaultEnneadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 9],
    /// Relationships between terms (36 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 36],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 9],
    pub coordinates: [Coordinates; 9],
    pub edges: [(usize, usize); 36],
}

impl Default for DefaultEnneadSystem {
    fn default() -> Self {
        Self {
            name: "Ennead",
            coherence_attribute: "Transformation",
            term_designation: "Needs Research",
            // Placeholder term characters - requires authentic Bennett research
            term_characters: [
                "Position1",
                "Position2",
                "Position3",
                "Position4",
                "Position5",
                "Position6",
                "Position7",
                "Position8",
                "Position9"
            ],
            connective_designation: "Needs Research",
            connective_characters: [
                // Each connective is uniquely named to maintain distinct relationships
                // First term connections
                ("Needs Research1", "Position1", "Position2"),
                ("Needs Research2", "Position1", "Position3"),
                ("Needs Research3", "Position1", "Position4"),
                ("Needs Research4", "Position1", "Position5"),
                ("Needs Research5", "Position1", "Position6"),
                ("Needs Research6", "Position1", "Position7"),
                ("Needs Research7", "Position1", "Position8"),
                ("Needs Research8", "Position1", "Position9"),
                // Second term connections
                ("Needs Research9", "Position2", "Position3"),
                ("Needs Research10", "Position2", "Position4"),
                ("Needs Research11", "Position2", "Position5"),
                ("Needs Research12", "Position2", "Position6"),
                ("Needs Research13", "Position2", "Position7"),
                ("Needs Research14", "Position2", "Position8"),
                ("Needs Research15", "Position2", "Position9"),
                // Third term connections
                ("Needs Research16", "Position3", "Position4"),
                ("Needs Research17", "Position3", "Position5"),
                ("Needs Research18", "Position3", "Position6"),
                ("Needs Research19", "Position3", "Position7"),
                ("Needs Research20", "Position3", "Position8"),
                ("Needs Research21", "Position3", "Position9"),
                // Fourth term connections
                ("Needs Research22", "Position4", "Position5"),
                ("Needs Research23", "Position4", "Position6"),
                ("Needs Research24", "Position4", "Position7"),
                ("Needs Research25", "Position4", "Position8"),
                ("Needs Research26", "Position4", "Position9"),
                // Fifth term connections
                ("Needs Research27", "Position5", "Position6"),
                ("Needs Research28", "Position5", "Position7"),
                ("Needs Research29", "Position5", "Position8"),
                ("Needs Research30", "Position5", "Position9"),
                // Sixth term connections
                ("Needs Research31", "Position6", "Position7"),
                ("Needs Research32", "Position6", "Position8"),
                ("Needs Research33", "Position6", "Position9"),
                // Seventh term connections
                ("Needs Research34", "Position7", "Position8"),
                ("Needs Research35", "Position7", "Position9"),
                // Eighth term connection
                ("Needs Research36", "Position8", "Position9"),
            ],
            source_attributions: [
                "Elementary Systematics",
                "Bennett's Work",
            ],
            
            // Geometry data for complete graph - regular 9-sided polygon
            // Starting index at top-right, going clockwise
            indexes: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            coordinates: [
                Coordinates { x: 0.64278760968, y: 0.76604444311, z: None },    // 0: top-right
                Coordinates { x: 0.98480775301, y: 0.17364817767, z: None },    // 1: right
                Coordinates { x: 0.86602540378, y: -0.5, z: None },             // 2: bottom-right
                Coordinates { x: 0.34202014333, y: -0.93969262079, z: None },   // 3: bottom
                Coordinates { x: -0.34202014333, y: -0.93969262079, z: None },  // 4: bottom-left
                Coordinates { x: -0.86602540378, y: -0.5, z: None },            // 5: left
                Coordinates { x: -0.98480775301, y: 0.17364817767, z: None },   // 6: top-left
                Coordinates { x: -0.64278760968, y: 0.76604444311, z: None },   // 7: top-left-2
                Coordinates { x: 0.0, y: 1.0, z: None },                        // 8: top
            ],
            edges: [
                // Complete graph edges for 9 vertices (36 edges)
                (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
                (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8),
                (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8),
                (3, 4), (3, 5), (3, 6), (3, 7), (3, 8),
                (4, 5), (4, 6), (4, 7), (4, 8),
                (5, 6), (5, 7), (5, 8),
                (6, 7), (6, 8),
                (7, 8),
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultEnneadSystem {
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
