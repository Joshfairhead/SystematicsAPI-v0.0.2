//! Connective designations represent the types of relationships
//! between terms in a system. These are fixed for each system level
//! and define how terms can interact or relate to each other.

/// The designation of connective types in a system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectiveDesignation {
    /// Monad connective designation - no connections between terms
    ConnectionlessUnity,
    /// Dyad connective designation - force between poles
    Force,
    /// Triad connective designation - acts between impulses
    Acts,
    /// Tetrad connective designation - interplays between sources
    Interplays,
    /// Pentad connective designation - mutualities between limits
    Mutualities,
    /// Hexad connective designation - steps between laws
    Steps,
    /// Heptad connective designation - intervals between states
    Intervals,
    /// Octad connective designation - components between elements
    Components,
    /// Term designation for systems 9-12 (requires research)
    Unknown,
}

impl ConnectiveDesignation {
    /// Get the string representation of this designation
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectiveDesignation::ConnectionlessUnity => "Connectionless Unity",
            ConnectiveDesignation::Force => "Force",
            ConnectiveDesignation::Acts => "Acts",
            ConnectiveDesignation::Interplays => "Interplays",
            ConnectiveDesignation::Mutualities => "Mutualities",
            ConnectiveDesignation::Steps => "Steps",
            ConnectiveDesignation::Intervals => "Intervals",
            ConnectiveDesignation::Components => "Components",
            ConnectiveDesignation::Unknown => "Unknown",
        }
    }
} 