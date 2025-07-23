use crate::core::traits::SystemData;
use crate::core::state_manager::Coordinates;

/// Core vocabulary configuration for the dodecad system
#[derive(Debug, Clone)]
pub struct DefaultDodecadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 12],
    /// Relationships between terms (66 bidirectional relationships)
    pub connective_characters: [(&'static str, &'static str, &'static str); 66],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
    
    // Geometry constants
    pub indexes: [usize; 12],
    pub coordinates: [Coordinates; 12],
    pub edges: [(usize, usize); 66],
}

impl Default for DefaultDodecadSystem {
    fn default() -> Self {
        Self {
            name: "Dodecad",
            coherence_attribute: "Perfection",
            term_designation: "Needs Research",
            term_characters: [ // Asymetrical layout based on a mapping from Tonys world Mandala to Bennett's work...
                "Autocracy",      // 0: top
                "Creativity",     // 1: top-right
                "Individuality",  // 2: right
                "Structure",      // 3: right
                "Wholeness",      // 4: bottom-right
                "Relatedness",    // 5: bottom
                "Subsistence",    // 6: bottom
                "Polarity",       // 7: bottom-left
                "Potentiality",   // 8: left
                "Repetition",     // 9: left
                "Pattern",        // 10: top-left
                "Domination"      // 11: top-left-2
            ],
            connective_designation: "Needs Research",
            connective_characters: [
                // First term connections (11)
                ("Needs Research1", "Autocracy", "Domination"),
                ("Needs Research2", "Autocracy", "Creativity"),
                ("Needs Research3", "Autocracy", "Pattern"),
                ("Needs Research4", "Autocracy", "Individuality"),
                ("Needs Research5", "Autocracy", "Structure"),
                ("Needs Research6", "Autocracy", "Repetition"),
                ("Needs Research7", "Autocracy", "Potentiality"),
                ("Needs Research8", "Autocracy", "Subsistence"),
                ("Needs Research9", "Autocracy", "Relatedness"),
                ("Needs Research10", "Autocracy", "Polarity"),
                ("Needs Research11", "Autocracy", "Wholeness"),
                // Second term connections (10)
                ("Needs Research12", "Domination", "Creativity"),
                ("Needs Research13", "Domination", "Pattern"),
                ("Needs Research14", "Domination", "Individuality"),
                ("Needs Research15", "Domination", "Structure"),
                ("Needs Research16", "Domination", "Repetition"),
                ("Needs Research17", "Domination", "Potentiality"),
                ("Needs Research18", "Domination", "Subsistence"),
                ("Needs Research19", "Domination", "Relatedness"),
                ("Needs Research20", "Domination", "Polarity"),
                ("Needs Research21", "Domination", "Wholeness"),
                // Third term connections (9)
                ("Needs Research22", "Creativity", "Pattern"),
                ("Needs Research23", "Creativity", "Individuality"),
                ("Needs Research24", "Creativity", "Structure"),
                ("Needs Research25", "Creativity", "Repetition"),
                ("Needs Research26", "Creativity", "Potentiality"),
                ("Needs Research27", "Creativity", "Subsistence"),
                ("Needs Research28", "Creativity", "Relatedness"),
                ("Needs Research29", "Creativity", "Polarity"),
                ("Needs Research30", "Creativity", "Wholeness"),
                // Fourth term connections (8)
                ("Needs Research31", "Pattern", "Individuality"),
                ("Needs Research32", "Pattern", "Structure"),
                ("Needs Research33", "Pattern", "Repetition"),
                ("Needs Research34", "Pattern", "Potentiality"),
                ("Needs Research35", "Pattern", "Subsistence"),
                ("Needs Research36", "Pattern", "Relatedness"),
                ("Needs Research37", "Pattern", "Polarity"),
                ("Needs Research38", "Pattern", "Wholeness"),
                // Fifth term connections (7)
                ("Needs Research39", "Individuality", "Structure"),
                ("Needs Research40", "Individuality", "Repetition"),
                ("Needs Research41", "Individuality", "Potentiality"),
                ("Needs Research42", "Individuality", "Subsistence"),
                ("Needs Research43", "Individuality", "Relatedness"),
                ("Needs Research44", "Individuality", "Polarity"),
                ("Needs Research45", "Individuality", "Wholeness"),
                // Sixth term connections (6)
                ("Needs Research46", "Structure", "Repetition"),
                ("Needs Research47", "Structure", "Potentiality"),
                ("Needs Research48", "Structure", "Subsistence"),
                ("Needs Research49", "Structure", "Relatedness"),
                ("Needs Research50", "Structure", "Polarity"),
                ("Needs Research51", "Structure", "Wholeness"),
                // Seventh term connections (5)
                ("Needs Research52", "Repetition", "Potentiality"),
                ("Needs Research53", "Repetition", "Subsistence"),
                ("Needs Research54", "Repetition", "Relatedness"),
                ("Needs Research55", "Repetition", "Polarity"),
                ("Needs Research56", "Repetition", "Wholeness"),
                // Eighth term connections (4)
                ("Needs Research57", "Potentiality", "Subsistence"),
                ("Needs Research58", "Potentiality", "Relatedness"),
                ("Needs Research59", "Potentiality", "Polarity"),
                ("Needs Research60", "Potentiality", "Wholeness"),
                // Ninth term connections (3)
                ("Needs Research61", "Subsistence", "Relatedness"),
                ("Needs Research62", "Subsistence", "Polarity"),
                ("Needs Research63", "Subsistence", "Wholeness"),
                // Tenth term connections (2)
                ("Needs Research64", "Relatedness", "Polarity"),
                ("Needs Research65", "Relatedness", "Wholeness"),
                // Eleventh term connection (1)
                ("Needs Research66", "Polarity", "Wholeness"),
            ],
            source_attributions: [
                "Elementary Systematics",
                "Bennett's Work",
            ],
            
            // Geometry data for complete graph - regular 12-sided polygon
            // Specific term-to-index mapping:
            // Autocracy(0), Domination(11), Creativity(1), Pattern(10), Individuality(2), Structure(3)
            // Repetition(9), Potentiality(8), Subsistence(6), Relatedness(5), Polarity(7), Wholeness(4)
            indexes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            coordinates: [
                Coordinates { x: 0.0, y: 1.0, z: None },                     // 0: Autocracy (top)
                Coordinates { x: 0.5, y: 0.86602540378, z: None },           // 1: Creativity (top-right)
                Coordinates { x: 0.86602540378, y: 0.5, z: None },           // 2: Individuality (right)
                Coordinates { x: 1.0, y: 0.0, z: None },                     // 3: Structure (right)
                Coordinates { x: 0.86602540378, y: -0.5, z: None },          // 4: Wholeness (bottom-right)
                Coordinates { x: 0.5, y: -0.86602540378, z: None },          // 5: Relatedness (bottom)
                Coordinates { x: 0.0, y: -1.0, z: None },                    // 6: Subsistence (bottom)
                Coordinates { x: -0.5, y: -0.86602540378, z: None },         // 7: Polarity (bottom-left)
                Coordinates { x: -0.86602540378, y: -0.5, z: None },         // 8: Potentiality (left)
                Coordinates { x: -1.0, y: 0.0, z: None },                    // 9: Repetition (left)
                Coordinates { x: -0.86602540378, y: 0.5, z: None },          // 10: Pattern (top-left)
                Coordinates { x: -0.5, y: 0.86602540378, z: None },          // 11: Domination (top-left-2)
            ],
            edges: [
                // Complete graph edges for 12 vertices (66 edges)
                (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10), (0, 11),
                (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (1, 10), (1, 11),
                (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11),
                (3, 4), (3, 5), (3, 6), (3, 7), (3, 8), (3, 9), (3, 10), (3, 11),
                (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), (4, 10), (4, 11),
                (5, 6), (5, 7), (5, 8), (5, 9), (5, 10), (5, 11),
                (6, 7), (6, 8), (6, 9), (6, 10), (6, 11),
                (7, 8), (7, 9), (7, 10), (7, 11),
                (8, 9), (8, 10), (8, 11),
                (9, 10), (9, 11),
                (10, 11),
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultDodecadSystem {
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