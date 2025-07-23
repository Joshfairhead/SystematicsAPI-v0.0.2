use crate::core::state_manager::{System, StateEvent, SystemId};
use crate::core::traits::SystemData;

/// Generic event-driven CRUD interface for system designations across all systems.
/// All mutations are performed via StateEvent and work with any SystemId.

pub struct Designations<S> {
    pub system: S,
}

// Read-only implementation
impl<'a> Designations<&'a System> {
    // TERM DESIGNATIONS
    /// Read term designation for any system
    pub fn read_term_designation(&self, system_id: SystemId) -> Option<&String> {
        self.system.term_designation.get(&system_id)
    }

    // CONNECTIVE DESIGNATIONS
    /// Read connective designation for any system
    pub fn read_connective_designation(&self, system_id: SystemId) -> Option<&String> {
        self.system.connective_designation.get(&system_id)
    }

    // COHERENCE ATTRIBUTES
    /// Read coherence attribute for any system
    pub fn read_coherence_attribute(&self, system_id: SystemId) -> Option<&String> {
        self.system.coherence_attributes.get(&system_id)
    }

    // SOURCE ATTRIBUTIONS
    /// Read source attribution for any system
    pub fn read_source_attribution(&self, system_id: SystemId) -> Option<&Vec<String>> {
        self.system.source_attributions.get(&system_id)
    }

    // SYSTEM NAMES
    /// Read system name for any system
    pub fn read_system_name(&self, system_id: SystemId) -> Option<&String> {
        self.system.system_names.get(&system_id)
    }

    // UTILITY
    /// Get all system IDs that have any designations
    pub fn get_systems_with_designations(&self) -> std::collections::HashSet<SystemId> {
        let mut systems = std::collections::HashSet::new();
        systems.extend(self.system.system_names.keys().cloned());
        systems.extend(self.system.coherence_attributes.keys().cloned());
        systems.extend(self.system.term_designation.keys().cloned());
        systems.extend(self.system.connective_designation.keys().cloned());
        systems.extend(self.system.source_attributions.keys().cloned());
        systems
    }

    /// Check if a system has any designations
    pub fn system_has_designations(&self, system_id: SystemId) -> bool {
        self.system.system_names.contains_key(&system_id) ||
        self.system.coherence_attributes.contains_key(&system_id) ||
        self.system.term_designation.contains_key(&system_id) ||
        self.system.connective_designation.contains_key(&system_id) ||
        self.system.source_attributions.contains_key(&system_id)
    }

    /// Get a summary of all designations for a system
    pub fn get_system_designation_summary(&self, system_id: &SystemId) -> SystemDesignationSummary {
        SystemDesignationSummary {
            system_id: system_id.clone(),
            system_name: self.read_system_name(system_id.clone()).cloned(),
            coherence_attribute: self.read_coherence_attribute(system_id.clone()).cloned(),
            term_designation: self.read_term_designation(system_id.clone()).cloned(),
            connective_designation: self.read_connective_designation(system_id.clone()).cloned(),
            source_attribution: self.read_source_attribution(system_id.clone()).cloned(),
        }
    }
}

// Mutable implementation
impl<'a> Designations<&'a mut System> {
    // TERM DESIGNATIONS
    /// Create term designation for any system
    pub fn create_term_designation(&mut self, system_id: SystemId, designation: String) {
        self.system.apply_event(StateEvent::CreateTermDesignation { system_id, designation });
    }

    /// Read term designation for any system
    pub fn read_term_designation(&self, system_id: SystemId) -> Option<&String> {
        self.system.term_designation.get(&system_id)
    }

    /// Update term designation for any system
    pub fn update_term_designation(&mut self, system_id: SystemId, designation: String) {
        self.system.apply_event(StateEvent::UpdateTermDesignation { system_id, designation });
    }

    /// Delete term designation for any system
    pub fn delete_term_designation(&mut self, system_id: SystemId) {
        self.system.apply_event(StateEvent::DeleteTermDesignation { system_id });
    }

    /// Load term designation from vocabulary for any system
    pub fn load_term_designation_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.create_term_designation(system_id, vocabulary.term_designation().to_string());
    }

    // CONNECTIVE DESIGNATIONS
    /// Create connective designation for any system
    pub fn create_connective_designation(&mut self, system_id: SystemId, designation: String) {
        self.system.apply_event(StateEvent::CreateConnectiveDesignation { system_id, designation });
    }

    /// Read connective designation for any system
    pub fn read_connective_designation(&self, system_id: SystemId) -> Option<&String> {
        self.system.connective_designation.get(&system_id)
    }

    /// Update connective designation for any system
    pub fn update_connective_designation(&mut self, system_id: SystemId, designation: String) {
        self.system.apply_event(StateEvent::UpdateConnectiveDesignation { system_id, designation });
    }

    /// Delete connective designation for any system
    pub fn delete_connective_designation(&mut self, system_id: SystemId) {
        self.system.apply_event(StateEvent::DeleteConnectiveDesignation { system_id });
    }

    /// Load connective designation from vocabulary for any system
    pub fn load_connective_designation_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.create_connective_designation(system_id, vocabulary.connective_designation().to_string());
    }

    // COHERENCE ATTRIBUTES
    /// Create coherence attribute for any system
    pub fn create_coherence_attribute(&mut self, system_id: SystemId, attribute: String) {
        self.system.apply_event(StateEvent::CreateCoherenceAttribute { system_id, attribute });
    }

    /// Read coherence attribute for any system
    pub fn read_coherence_attribute(&self, system_id: SystemId) -> Option<&String> {
        self.system.coherence_attributes.get(&system_id)
    }

    /// Update coherence attribute for any system
    pub fn update_coherence_attribute(&mut self, system_id: SystemId, attribute: String) {
        self.system.apply_event(StateEvent::UpdateCoherenceAttribute { system_id, attribute });
    }

    /// Delete coherence attribute for any system
    pub fn delete_coherence_attribute(&mut self, system_id: SystemId) {
        self.system.apply_event(StateEvent::DeleteCoherenceAttribute { system_id });
    }

    /// Load coherence attribute from vocabulary for any system
    pub fn load_coherence_attribute_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.create_coherence_attribute(system_id, vocabulary.coherence_attribute().to_string());
    }

    // SOURCE ATTRIBUTIONS
    /// Create source attribution for any system
    pub fn create_source_attribution(&mut self, system_id: SystemId, attribution: Vec<String>) {
        self.system.apply_event(StateEvent::CreateSourceAttribution { system_id, source: attribution });
    }

    /// Read source attribution for any system
    pub fn read_source_attribution(&self, system_id: SystemId) -> Option<&Vec<String>> {
        self.system.source_attributions.get(&system_id)
    }

    /// Update source attribution for any system
    pub fn update_source_attribution(&mut self, system_id: SystemId, attribution: Vec<String>) {
        self.system.apply_event(StateEvent::UpdateSourceAttribution { system_id, source: attribution });
    }

    /// Delete source attribution for any system
    pub fn delete_source_attribution(&mut self, system_id: SystemId) {
        self.system.apply_event(StateEvent::DeleteSourceAttribution { system_id });
    }

    /// Load source attribution from vocabulary for any system
    pub fn load_source_attribution_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        let attribution: Vec<String> = vocabulary.source_attributions().iter().map(|s| s.to_string()).collect();
        self.create_source_attribution(system_id, attribution);
    }

    // SYSTEM NAMES
    /// Create system name for any system
    pub fn create_system_name(&mut self, system_id: SystemId, name: String) {
        self.system.apply_event(StateEvent::CreateSystemName { system_id, name });
    }

    /// Read system name for any system
    pub fn read_system_name(&self, system_id: SystemId) -> Option<&String> {
        self.system.system_names.get(&system_id)
    }

    /// Update system name for any system
    pub fn update_system_name(&mut self, system_id: SystemId, name: String) {
        self.system.apply_event(StateEvent::UpdateSystemName { system_id, name });
    }

    /// Delete system name for any system
    pub fn delete_system_name(&mut self, system_id: SystemId) {
        self.system.apply_event(StateEvent::DeleteSystemName { system_id });
    }

    /// Load system name from vocabulary for any system
    pub fn load_system_name_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.create_system_name(system_id, vocabulary.system_name().to_string());
    }

    // BULK OPERATIONS
    /// Load all designations from vocabulary for any system
    pub fn load_all_designations_from_vocabulary<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.load_system_name_from_vocabulary(system_id.clone(), vocabulary);
        self.load_coherence_attribute_from_vocabulary(system_id.clone(), vocabulary);
        self.load_term_designation_from_vocabulary(system_id.clone(), vocabulary);
        self.load_connective_designation_from_vocabulary(system_id.clone(), vocabulary);
        self.load_source_attribution_from_vocabulary(system_id, vocabulary);
    }

    /// Delete all designations for any system
    pub fn delete_all_designations_for_system(&mut self, system_id: SystemId) {
        self.delete_system_name(system_id.clone());
        self.delete_coherence_attribute(system_id.clone());
        self.delete_term_designation(system_id.clone());
        self.delete_connective_designation(system_id.clone());
        self.delete_source_attribution(system_id);
    }

    /// Reset all designations to canonical vocabulary for any system
    pub fn reset_all_designations_to_canonical<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.delete_all_designations_for_system(system_id.clone());
        self.load_all_designations_from_vocabulary(system_id, vocabulary);
    }

    // UTILITY (implement directly, don't delegate)
    /// Get all system IDs that have any designations
    pub fn get_systems_with_designations(&self) -> std::collections::HashSet<SystemId> {
        let mut systems = std::collections::HashSet::new();
        systems.extend(self.system.system_names.keys().cloned());
        systems.extend(self.system.coherence_attributes.keys().cloned());
        systems.extend(self.system.term_designation.keys().cloned());
        systems.extend(self.system.connective_designation.keys().cloned());
        systems.extend(self.system.source_attributions.keys().cloned());
        systems
    }

    /// Check if a system has any designations
    pub fn system_has_designations(&self, system_id: SystemId) -> bool {
        self.system.system_names.contains_key(&system_id) ||
        self.system.coherence_attributes.contains_key(&system_id) ||
        self.system.term_designation.contains_key(&system_id) ||
        self.system.connective_designation.contains_key(&system_id) ||
        self.system.source_attributions.contains_key(&system_id)
    }

    /// Get a summary of all designations for a system
    pub fn get_system_designation_summary(&self, system_id: &SystemId) -> SystemDesignationSummary {
        SystemDesignationSummary {
            system_id: system_id.clone(),
            system_name: self.read_system_name(system_id.clone()).cloned(),
            coherence_attribute: self.read_coherence_attribute(system_id.clone()).cloned(),
            term_designation: self.read_term_designation(system_id.clone()).cloned(),
            connective_designation: self.read_connective_designation(system_id.clone()).cloned(),
            source_attribution: self.read_source_attribution(system_id.clone()).cloned(),
        }
    }
}

/// Summary of all designations for a system
#[derive(Debug, Clone)]
pub struct SystemDesignationSummary {
    pub system_id: SystemId,
    pub system_name: Option<String>,
    pub coherence_attribute: Option<String>,
    pub term_designation: Option<String>,
    pub connective_designation: Option<String>,
    pub source_attribution: Option<Vec<String>>,
} 