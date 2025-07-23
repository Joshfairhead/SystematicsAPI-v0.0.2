pub mod monad;
pub mod dyad;
pub mod triad;
pub mod tetrad;
pub mod pentad;
pub mod hexad;
pub mod heptad;
pub mod octad;
pub mod ennead;
pub mod decad;
pub mod undecad;
pub mod dodecad;

use crate::core::traits::SemanticData;

// Re-export all semantic structs in order
pub use monad::MonadSemantics;
pub use dyad::DyadSemantics;
pub use triad::TriadSemantics;
pub use tetrad::TetradSemantics;
pub use pentad::PentadSemantics;
pub use hexad::HexadSemantics;
pub use heptad::HeptadSemantics;
pub use octad::OctadSemantics;
pub use ennead::EnneadSemantics;
pub use decad::DecadSemantics;
pub use undecad::UndecadSemantics;
pub use dodecad::DodecadSemantics;

// Implement SemanticData for all semantic structs
impl SemanticData for MonadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
} 

impl SemanticData for DyadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for TriadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for TetradSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for PentadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for HexadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for HeptadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for OctadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for EnneadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for DecadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for UndecadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}

impl SemanticData for DodecadSemantics {
    fn system_name(&self) -> &'static str { Self::NAME }
    fn coherence_attribute(&self) -> &'static str { Self::COHERENCE_ATTRIBUTE }
    fn term_designation(&self) -> &'static str { Self::TERM_DESIGNATION }
    fn term_characters(&self) -> &[&'static str] { &Self::TERM_CHARACTERS }
    fn connective_designation(&self) -> &'static str { Self::CONNECTIVE_DESIGNATION }
    fn connective_characters(&self) -> &[(&'static str, &'static str, &'static str)] { &Self::CONNECTIVE_CHARACTERS }
    fn source_attributions(&self) -> &[&'static str] { &Self::SOURCE_ATTRIBUTIONS }
}