pub struct HeptadSemantics;

impl HeptadSemantics {
    pub const NAME: &'static str = "Heptad";
    pub const COHERENCE_ATTRIBUTE: &'static str = "Generation";
    pub const TERM_DESIGNATION: &'static str = "States";
    pub const CONNECTIVE_DESIGNATION: &'static str = "Intervals";
    pub const TERM_CHARACTERS: [&'static str; 7] = ["Insight", "Research", "Design", "Synthesis", "Application", "Delivery", "Value"];
    pub const CONNECTIVE_CHARACTERS: [(&'static str, &'static str, &'static str); 21] = [
        ("Needs Research", "Insight", "Research"),
        ("Needs Research", "Insight", "Design"),
        ("Needs Research", "Insight", "Synthesis"),
        ("Needs Research", "Insight", "Application"),
        ("Needs Research", "Insight", "Delivery"),
        ("Needs Research", "Insight", "Value"),
        ("Needs Research", "Research", "Design"),
        ("Needs Research", "Research", "Synthesis"),
        ("Needs Research", "Research", "Application"),
        ("Needs Research", "Research", "Delivery"),
        ("Needs Research", "Research", "Value"),
        ("Needs Research", "Design", "Synthesis"),
        ("Needs Research", "Design", "Application"),
        ("Needs Research", "Design", "Delivery"),
        ("Needs Research", "Design", "Value"),
        ("Needs Research", "Synthesis", "Application"),
        ("Needs Research", "Synthesis", "Delivery"),
        ("Needs Research", "Synthesis", "Value"),
        ("Needs Research", "Application", "Delivery"),
        ("Needs Research", "Application", "Value"),
        ("Needs Research", "Delivery", "Value"),
    ];
    pub const SOURCE_ATTRIBUTIONS: [&'static str; 2] = ["Elementary Systematics", "Bennett's Work"];
} 