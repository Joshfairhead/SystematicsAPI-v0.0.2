//! Coherence Attributes
//! 
//! Each n-term system has its own characteristic coherence attribute
//! that describes its primary organizing principle.

/// Coherence attributes for each n-term system as defined in Bennett's vocabulary
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoherenceAttribute {
    Universality,
    Complementarity,
    Dynamism,
    ActivityField,
    SignificanceAndPotential,
    Coalescence,
    Generation,
    SelfSufficiency,
    Transformation,
    IntrinsicHarmony,
    ArticulateSymmetry,
    Harmony,
}

impl CoherenceAttribute {
    /// Get the name of this coherence attribute
    pub fn name(&self) -> String {
        match self {
            CoherenceAttribute::Universality => "Universality".to_string(),
            CoherenceAttribute::Complementarity => "Complementarity".to_string(),
            CoherenceAttribute::Dynamism => "Dynamism".to_string(),
            CoherenceAttribute::ActivityField => "Activity Field".to_string(),
            CoherenceAttribute::SignificanceAndPotential => "Significance and Potential".to_string(),
            CoherenceAttribute::Coalescence => "Coalescence".to_string(),
            CoherenceAttribute::Generation => "Generation".to_string(),
            CoherenceAttribute::SelfSufficiency => "Self-Sufficiency".to_string(),
            CoherenceAttribute::Transformation => "Transformation".to_string(),
            CoherenceAttribute::IntrinsicHarmony => "Intrinsic Harmony".to_string(),
            CoherenceAttribute::ArticulateSymmetry => "Articulate Symmetry".to_string(),
            CoherenceAttribute::Harmony => "Harmony".to_string(),
        }
    }
} 