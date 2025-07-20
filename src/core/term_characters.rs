use crate::core::state_manager::{System, StateEvent, SystemId, Index};
use crate::core::default_system_data::DefaultSystemData;

/// Term data structure representing a single term in a system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Term {
    pub index: usize,
    pub character: String,
}

/// Generic event-driven CRUD interface for term characters across all systems.
/// All mutations are performed via StateEvent and work with any SystemId.

pub struct TermCharacters<S> {
    pub system: S,
}

// Read-only implementation
impl<'a> TermCharacters<&'a System> {
    // READ
    /// Read a single term character by index for any system
    pub fn read_term_character_by_index(&self, system_id: SystemId, index: Index) -> Option<&Term> {
        self.system.terms.get(&(system_id, index))
    }

    /// Read all term characters for a specific system as a vector
    pub fn read_term_characters_as_vector(&self, system_id: SystemId) -> Vec<(Index, &Term)> {
        self.system.terms.iter()
            .filter_map(|((sid, idx), term)| if sid == &system_id { Some((*idx, term)) } else { None })
            .collect()
    }

    /// Get all term characters for a system as a HashMap
    pub fn read_term_characters_as_map(&self, system_id: SystemId) -> std::collections::HashMap<Index, &Term> {
        self.system.terms.iter()
            .filter_map(|((sid, idx), term)| if sid == &system_id { Some((*idx, term)) } else { None })
            .collect()
    }

    // UTILITY
    /// Get the count of terms for a specific system
    pub fn term_count_for_system(&self, system_id: SystemId) -> usize {
        self.system.terms.iter()
            .filter(|((sid, _), _)| sid == &system_id)
            .count()
    }

    /// Check if a term exists at a specific index for a system
    pub fn term_exists(&self, system_id: SystemId, index: Index) -> bool {
        self.system.terms.contains_key(&(system_id, index))
    }

    /// Get all system IDs that have terms
    pub fn get_systems_with_terms(&self) -> std::collections::HashSet<SystemId> {
        self.system.terms.keys()
            .map(|(sid, _)| sid.clone())
            .collect()
    }
}

// Mutable implementation
impl<'a> TermCharacters<&'a mut System> {
    // CREATE
    /// Create a single term character by index for any system
    pub fn create_term_character_by_index(&mut self, system_id: SystemId, index: Index, label: &str) {
        self.system.apply_event(StateEvent::CreateTerm {
            system_id,
            index,
            character: label.to_string().to_lowercase(),
        });
    }

    /// Create multiple term characters from a vector for any system
    pub fn create_term_characters_by_vector(&mut self, system_id: SystemId, terms: Vec<(Index, String)>) {
        for (index, character) in terms {
            self.create_term_character_by_index(system_id.clone(), index, &character);
        }
    }

    /// Load term characters from a vocabulary implementation
    pub fn load_from_vocabulary<T: DefaultSystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        let terms: Vec<_> = vocabulary.term_characters()
            .iter()
            .enumerate()
            .map(|(i, term)| (i as Index, term.to_string()))
            .collect();
        self.create_term_characters_by_vector(system_id, terms);
    }

    // READ (implement directly, don't delegate)
    /// Read a single term character by index for any system
    pub fn read_term_character_by_index(&self, system_id: SystemId, index: Index) -> Option<&Term> {
        self.system.terms.get(&(system_id, index))
    }

    /// Read all term characters for a specific system as a vector
    pub fn read_term_characters_as_vector(&self, system_id: SystemId) -> Vec<(Index, &Term)> {
        self.system.terms.iter()
            .filter_map(|((sid, idx), term)| if sid == &system_id { Some((*idx, term)) } else { None })
            .collect()
    }

    /// Get all term characters for a system as a HashMap
    pub fn read_term_characters_as_map(&self, system_id: SystemId) -> std::collections::HashMap<Index, &Term> {
        self.system.terms.iter()
            .filter_map(|((sid, idx), term)| if sid == &system_id { Some((*idx, term)) } else { None })
            .collect()
    }

    // UPDATE
    /// Update a single term character by index for any system
    pub fn update_term_character_by_index(&mut self, system_id: SystemId, index: Index, character: String) {
        self.system.apply_event(StateEvent::UpdateTermCharacter { system_id, index, character });
    }

    /// Update multiple term characters from a vector for any system
    pub fn update_term_characters_by_vector(&mut self, system_id: SystemId, terms: Vec<(Index, String)>) {
        for (index, character) in terms {
            self.update_term_character_by_index(system_id.clone(), index, character);
        }
    }

    // DELETE
    /// Delete a single term by index for any system
    pub fn delete_term_by_index(&mut self, system_id: SystemId, index: Index) {
        self.system.apply_event(StateEvent::DeleteTerm { system_id, index });
    }

    /// Delete multiple terms by vector of indices for any system
    pub fn delete_terms_by_vector(&mut self, system_id: SystemId, indices: Vec<Index>) {
        for index in indices {
            self.delete_term_by_index(system_id.clone(), index);
        }
    }

    /// Delete all terms for a specific system
    pub fn delete_all_terms_for_system(&mut self, system_id: SystemId) {
        let indices: Vec<_> = self.system.terms.keys()
            .filter_map(|(sid, idx)| if sid == &system_id { Some(*idx) } else { None })
            .collect();
        self.delete_terms_by_vector(system_id, indices);
    }

    // RESET
    /// Reset terms to default values for any system
    pub fn reset_to_default(&mut self, system_id: SystemId, default_terms: &[(Index, &str)]) {
        let indices: Vec<_> = self.system.terms.keys()
            .filter_map(|(sid, idx)| if sid == &system_id { Some(*idx) } else { None })
            .collect();
        self.delete_terms_by_vector(system_id.clone(), indices);
        let terms = default_terms.iter().map(|(idx, name)| {
            (*idx, name.to_string())
        }).collect();
        self.create_term_characters_by_vector(system_id, terms);
    }

    /// Reset terms to canonical vocabulary for any system
    pub fn reset_to_canonical<T: DefaultSystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.delete_all_terms_for_system(system_id.clone());
        self.load_from_vocabulary(system_id, vocabulary);
    }

    // UTILITY (implement directly, don't delegate)
    /// Get the count of terms for a specific system
    pub fn term_count_for_system(&self, system_id: SystemId) -> usize {
        self.system.terms.iter()
            .filter(|((sid, _), _)| sid == &system_id)
            .count()
    }

    /// Check if a term exists at a specific index for a system
    pub fn term_exists(&self, system_id: SystemId, index: Index) -> bool {
        self.system.terms.contains_key(&(system_id, index))
    }

    /// Get all system IDs that have terms
    pub fn get_systems_with_terms(&self) -> std::collections::HashSet<SystemId> {
        self.system.terms.keys()
            .map(|(sid, _)| sid.clone())
            .collect()
    }
} 