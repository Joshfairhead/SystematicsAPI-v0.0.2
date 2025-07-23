use crate::core::traits::SystemData;

/// Core vocabulary configuration for the dyad system
#[derive(Debug, Clone)]
pub struct DefaultDyadSystem {
    /// Name of the system
    pub name: &'static str,
    /// Primary organizing principle
    pub coherence_attribute: &'static str,
    /// Nature of the term positions
    pub term_designation: &'static str,
    /// Nature of the connective positions
    pub connective_designation: &'static str,
    /// Ordered vocabulary for each position
    pub term_characters: [&'static str; 2],
    /// Relationships between terms
    pub connective_characters: [(&'static str, &'static str, &'static str); 1],
    /// Source attributions
    pub source_attributions: [&'static str; 2],
}

impl Default for DefaultDyadSystem {
    fn default() -> Self {
        Self {
            name: "Dyad",
            coherence_attribute: "Complimentarity",
            term_designation: "Poles",
            term_characters: ["Essence", "Existence"],
            connective_designation: "Force",
            connective_characters: [("Force1", "Essence", "Existence")], // Essence → Existence
            source_attributions: [
                "Elementary Systematics",
                "Bennett's Work",
            ],
        }
    }
}

/// Access the data from the vocabulary struct
impl SystemData for DefaultDyadSystem {
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
}
