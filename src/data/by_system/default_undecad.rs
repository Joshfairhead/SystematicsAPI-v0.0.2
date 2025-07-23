use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the undecad system
/// Note: This is a placeholder implementation as the term characters require authentic Bennett research
#[derive(Debug, Clone)]
pub struct DefaultUndecadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 11],
    /// Relationships between terms (55 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 55],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 11],
    pub coordinates: [Coordinates; 11],
    pub edges: [(usize, usize); 55],
}

impl Default for DefaultUndecadSystem {
    fn default() -> Self {
        Self {
            name: "Undecad",
            coherence_attribute: "Articulate Symmetry",
            term_designation: "Needs Research",
            term_characters: [
                "Position1",
                "Position2",
                "Position3",
                "Position4",
                "Position5",
                "Position6",
                "Position7",
                "Position8",
                "Position9",
                "Position10",
                "Position11"
            ],
            connective_designation: "Needs Research",
            connective_characters: [
                // Each connective is uniquely named to maintain distinct relationships
                // First term connections (10)
                ("Needs Research1", "Position1", "Position2"),
                ("Needs Research2", "Position1", "Position3"),
                ("Needs Research3", "Position1", "Position4"),
                ("Needs Research4", "Position1", "Position5"),
                ("Needs Research5", "Position1", "Position6"),
                ("Needs Research6", "Position1", "Position7"),
                ("Needs Research7", "Position1", "Position8"),
                ("Needs Research8", "Position1", "Position9"),
                ("Needs Research9", "Position1", "Position10"),
                ("Needs Research10", "Position1", "Position11"),
                // Second term connections (9)
                ("Needs Research11", "Position2", "Position3"),
                ("Needs Research12", "Position2", "Position4"),
                ("Needs Research13", "Position2", "Position5"),
                ("Needs Research14", "Position2", "Position6"),
                ("Needs Research15", "Position2", "Position7"),
                ("Needs Research16", "Position2", "Position8"),
                ("Needs Research17", "Position2", "Position9"),
                ("Needs Research18", "Position2", "Position10"),
                ("Needs Research19", "Position2", "Position11"),
                // Third term connections (8)
                ("Needs Research20", "Position3", "Position4"),
                ("Needs Research21", "Position3", "Position5"),
                ("Needs Research22", "Position3", "Position6"),
                ("Needs Research23", "Position3", "Position7"),
                ("Needs Research24", "Position3", "Position8"),
                ("Needs Research25", "Position3", "Position9"),
                ("Needs Research26", "Position3", "Position10"),
                ("Needs Research27", "Position3", "Position11"),
                // Fourth term connections (7)
                ("Needs Research28", "Position4", "Position5"),
                ("Needs Research29", "Position4", "Position6"),
                ("Needs Research30", "Position4", "Position7"),
                ("Needs Research31", "Position4", "Position8"),
                ("Needs Research32", "Position4", "Position9"),
                ("Needs Research33", "Position4", "Position10"),
                ("Needs Research34", "Position4", "Position11"),
                // Fifth term connections (6)
                ("Needs Research35", "Position5", "Position6"),
                ("Needs Research36", "Position5", "Position7"),
                ("Needs Research37", "Position5", "Position8"),
                ("Needs Research38", "Position5", "Position9"),
                ("Needs Research39", "Position5", "Position10"),
                ("Needs Research40", "Position5", "Position11"),
                // Sixth term connections (5)
                ("Needs Research41", "Position6", "Position7"),
                ("Needs Research42", "Position6", "Position8"),
                ("Needs Research43", "Position6", "Position9"),
                ("Needs Research44", "Position6", "Position10"),
                ("Needs Research45", "Position6", "Position11"),
                // Seventh term connections (4)
                ("Needs Research46", "Position7", "Position8"),
                ("Needs Research47", "Position7", "Position9"),
                ("Needs Research48", "Position7", "Position10"),
                ("Needs Research49", "Position7", "Position11"),
                // Eighth term connections (3)
                ("Needs Research50", "Position8", "Position9"),
                ("Needs Research51", "Position8", "Position10"),
                ("Needs Research52", "Position8", "Position11"),
                // Ninth term connections (2)
                ("Needs Research53", "Position9", "Position10"),
                ("Needs Research54", "Position9", "Position11"),
                // Tenth term connection (1)
                ("Needs Research55", "Position10", "Position11"),
            ],
            source_attributions: [
                "Elementary Systematics",
                "Bennett's Work",
            ],
            
            // Geometry data for complete graph - regular 11-sided polygon
            // Starting index at top, going clockwise
            indexes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            coordinates: [
                Coordinates { x: 0.0, y: 1.0, z: None },                     // 0: top
                Coordinates { x: 0.54064081746, y: 0.84125353283, z: None }, // 1: top-right
                Coordinates { x: 0.90963199535, y: 0.41541501300, z: None }, // 2: right
                Coordinates { x: 0.98982144188, y: -0.14231483827, z: None }, // 3: bottom-right
                Coordinates { x: 0.75574957435, y: -0.65486073395, z: None }, // 4: bottom
                Coordinates { x: 0.28173255684, y: -0.95949297361, z: None }, // 5: bottom
                Coordinates { x: -0.28173255684, y: -0.95949297361, z: None }, // 6: bottom-left
                Coordinates { x: -0.75574957435, y: -0.65486073395, z: None }, // 7: left
                Coordinates { x: -0.98982144188, y: -0.14231483827, z: None }, // 8: top-left
                Coordinates { x: -0.90963199535, y: 0.41541501300, z: None },  // 9: top-left-2
                Coordinates { x: -0.54064081746, y: 0.84125353283, z: None },  // 10: top-left-3
            ],
            edges: [
                // Complete graph edges for 11 vertices (55 edges)
                (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10),
                (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10),
                (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10),
                (3, 4), (3, 5), (3, 6), (3, 7), (3, 8), (3, 9), (3, 10),
                (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), (4, 10),
                (5, 6), (5, 7), (5, 8), (5, 9), (5, 10),
                (6, 7), (6, 8), (6, 9), (6, 10),
                (7, 8), (7, 9), (7, 10),
                (8, 9), (8, 10),
                (9, 10),
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultUndecadSystem {
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
