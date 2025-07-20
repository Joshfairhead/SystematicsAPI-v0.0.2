//! Connective characters represent the specific relationships
//! between terms in a system. Unlike connective designations which are
//! fixed categories, characters describe the actual relationships that
//! exist between terms.

/// Characters for Monadic system (no connections)
#[derive(Debug, Clone, PartialEq)]
pub enum MonadicConnectiveCharacters {
    None,
}

/// Characters for Dyadic system
#[derive(Debug, Clone, PartialEq)]
pub enum DyadicConnectiveCharacters {
    EssenceToExistence,
}

/// Characters for Triadic system
#[derive(Debug, Clone, PartialEq)]
pub enum TriadicConnectiveCharacters {
    /// Act from Will to Function
    WillToFunction,
    /// Act from Function to Being
    FunctionToBeing,
    /// Act from Being to Will
    BeingToWill,
}

/// Characters for Tetradic system
#[derive(Debug, Clone, PartialEq)]
pub enum TetradicConnectiveCharacters {
    /// Interplay between Ideal and Directive
    ReceptiveRegard,
    /// Interplay between Ideal and Instrumental
    EffectualCompatibility,
    /// Interplay between Ideal and Ground
    MotivationalImperative,
    /// Interplay between Directive and Instrumental
    DemonstrableActivity,
    /// Interplay between Directive and Ground
    MaterialMastery,
    /// Interplay between Instrumental and Ground
    TechnicalPower,
}

/// Characters for Pentadic system
#[derive(Debug, Clone, PartialEq)]
pub enum PentadicConnectiveCharacters {
    /// Mutuality of potential range
    RangeOfPotential,
    /// Mutuality of significance range
    RangeOfSignificance,
    /// Mutuality of aspiration
    Aspiration,
    /// Mutuality of operation
    Operation,
    /// Mutuality of output
    Output,
    /// Mutuality of input
    Input,
    /// Mutuality of qualitative matching
    QualitativeMatch,
    /// Mutuality of quantitative matching
    QuantitativeMatch,
    /// Mutuality of form
    Form,
    /// Mutuality of function
    Function,
}

/// Characters for Hexadic system
#[derive(Debug, Clone, PartialEq)]
pub enum HexadicConnectiveCharacters {
    /// Bidirectional step relationships
    BidirectionalStep,
}

/// Characters for Heptadic system
#[derive(Debug, Clone, PartialEq)]
pub enum HeptadicConnectiveCharacters {
    /// Bidirectional interval relationships
    BidirectionalInterval,
}

/// Characters for Octadic system
#[derive(Debug, Clone, PartialEq)]
pub enum OctadicConnectiveCharacters {
    /// Bidirectional component relationships
    BidirectionalComponent,
}

/// A connective character represents a specific relationship between terms
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectiveCharacters {
    /// Monadic connective character (none)
    Monadic(MonadicConnectiveCharacters),
    /// Dyadic connective character
    Dyadic(DyadicConnectiveCharacters),
    /// Triadic connective character
    Triadic(TriadicConnectiveCharacters),
    /// Tetradic connective character
    Tetradic(TetradicConnectiveCharacters),
    /// Pentadic connective character
    Pentadic(PentadicConnectiveCharacters),
    /// Hexadic connective character
    Hexadic(HexadicConnectiveCharacters),
    /// Heptadic connective character
    Heptadic(HeptadicConnectiveCharacters),
    /// Octadic connective character
    Octadic(OctadicConnectiveCharacters),
    /// Custom character (for systems 9-12 or custom implementations)
    Custom(String),
}

impl ConnectiveCharacters {
    /// Get the string representation of this character
    pub fn as_str(&self) -> String {
        match self {
            ConnectiveCharacters::Monadic(c) => match c {
                MonadicConnectiveCharacters::None => "None".to_string(),
            },
            ConnectiveCharacters::Dyadic(c) => match c {
                DyadicConnectiveCharacters::EssenceToExistence => "Essence → Existence".to_string(),
            },
            ConnectiveCharacters::Triadic(c) => match c {
                TriadicConnectiveCharacters::WillToFunction => "Will → Function".to_string(),
                TriadicConnectiveCharacters::FunctionToBeing => "Function → Being".to_string(),
                TriadicConnectiveCharacters::BeingToWill => "Being → Will".to_string(),
            },
            ConnectiveCharacters::Tetradic(c) => match c {
                TetradicConnectiveCharacters::ReceptiveRegard => "Receptive Regard".to_string(),
                TetradicConnectiveCharacters::EffectualCompatibility => "Effectual Compatibility".to_string(),
                TetradicConnectiveCharacters::MotivationalImperative => "Motivational Imperative".to_string(),
                TetradicConnectiveCharacters::DemonstrableActivity => "Demonstrable Activity".to_string(),
                TetradicConnectiveCharacters::MaterialMastery => "Material Mastery".to_string(),
                TetradicConnectiveCharacters::TechnicalPower => "Technical Power".to_string(),
            },
            ConnectiveCharacters::Pentadic(c) => match c {
                PentadicConnectiveCharacters::RangeOfPotential => "Range of Potential".to_string(),
                PentadicConnectiveCharacters::RangeOfSignificance => "Range of Significance".to_string(),
                PentadicConnectiveCharacters::Aspiration => "Aspiration".to_string(),
                PentadicConnectiveCharacters::Operation => "Operation".to_string(),
                PentadicConnectiveCharacters::Output => "Output".to_string(),
                PentadicConnectiveCharacters::Input => "Input".to_string(),
                PentadicConnectiveCharacters::QualitativeMatch => "Qualitative Match".to_string(),
                PentadicConnectiveCharacters::QuantitativeMatch => "Quantitative Match".to_string(),
                PentadicConnectiveCharacters::Form => "Form".to_string(),
                PentadicConnectiveCharacters::Function => "Function".to_string(),
            },
            ConnectiveCharacters::Hexadic(c) => match c {
                HexadicConnectiveCharacters::BidirectionalStep => "Bidirectional Step".to_string(),
            },
            ConnectiveCharacters::Heptadic(c) => match c {
                HeptadicConnectiveCharacters::BidirectionalInterval => "Bidirectional Interval".to_string(),
            },
            ConnectiveCharacters::Octadic(c) => match c {
                OctadicConnectiveCharacters::BidirectionalComponent => "Bidirectional Component".to_string(),
            },
            ConnectiveCharacters::Custom(s) => s.clone(),
        }
    }
} 