pub struct TriadSemantics;

impl TriadSemantics {
    pub const NAME: &'static str = "Triad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Dynamism";
    pub const TERM_DESIGNATION: &'static str = "Impulses";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Acts";
    pub const TERM_CHARACTERS: [&'static str; 3] = ["Will", "Function", "Being"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 3] = [
        ("Act1", "Will", "Function"),
        ("Act2", "Function", "Being"),
        ("Act3", "Being", "Will"),
    ];
    pub const SOURCE_ATTRIBUTIONS: [&'static str; 2] = ["Elementary Systematics", "Bennett's Work"];
} 