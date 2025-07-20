//! Term designations represent the abstract categories that describe
//! the nature of term positions within systematic structures.
//! These are fixed for each system level (e.g., Monad has Totality,
//! Dyad has Poles, etc.).

/// The designation of a term's position in a system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermDesignation {
    /// Monad term designation
    Totality,
    /// Dyad term designation
    Poles,
    /// Triad term designation
    Impulses,
    /// Tetrad term designation
    Sources,
    /// Pentad term designation
    Limits,
    /// Hexad term designation
    Laws,
    /// Heptad term designation
    States,
    /// Octad term designation
    Elements,
    /// Term designation for systems 9-12 (requires research)
    Unknown,
}

impl TermDesignation {
    /// Get the string representation of this designation
    pub fn as_str(&self) -> &'static str {
        match self {
            TermDesignation::Totality => "Totality",
            TermDesignation::Poles => "Poles",
            TermDesignation::Impulses => "Impulses",
            TermDesignation::Sources => "Sources",
            TermDesignation::Limits => "Limits",
            TermDesignation::Laws => "Laws",
            TermDesignation::States => "States",
            TermDesignation::Elements => "Elements",
            TermDesignation::Unknown => "Unknown",
        }
    }
} 