pub struct DyadSemantics;

impl DyadSemantics {
    pub const NAME: &'static str = "Dyad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Complimentarity";
    pub const TERM_DESIGNATION: &'static str = "Poles";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Force";
    pub const TERM_CHARACTERS: [&'static str; 2] = ["Essence", "Existence"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 1] = [
        ("Force1", "Essence", "Existence"),
    ];
    pub const SOURCE_ATTRIBUTIONS: [&'static str; 2] = ["Elementary Systematics", "Bennett's Work"];
} 