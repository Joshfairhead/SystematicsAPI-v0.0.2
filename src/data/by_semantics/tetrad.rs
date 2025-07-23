pub struct TetradSemantics;

impl TetradSemantics {
    pub const NAME: &'static str = "Tetrad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Activity Field";
    pub const TERM_DESIGNATION: &'static str = "Sources";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Interplays";
    pub const TERM_CHARACTERS: [&'static str; 4] = ["Ideal", "Directive", "Instrumental", "Ground"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 6] = [
        ("Receptive Regard", "Ideal", "Directive"),
        ("Effectual Compatibility", "Ideal", "Instrumental"),
        ("Motivational Imperative", "Ideal", "Ground"),
        ("Demonstrable Activity", "Directive", "Instrumental"),
        ("Material Mastery", "Directive", "Ground"),
        ("Technical Power", "Instrumental", "Ground"),
    ];
    pub const SOURCE_ATTRIBUTIONS: [&'static str; 2] = ["Elementary Systematics", "Bennett's Work"];
} 