use crate::core::state_manager::{System, StateEvent, SystemId, Index, IndexPair};
use crate::core::default_system_data::DefaultSystemData;

/// Connective data structure representing a connection between two terms in a system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Connective {
    pub indices: (usize, usize),
    pub character: String,
}

/// Generic event-driven CRUD interface for connective characters across all systems.
/// All mutations are performed via StateEvent and work with any SystemId.

pub struct ConnectiveCharacters<S> {
    pub system: S,
}

// Read-only implementation
impl<'a> ConnectiveCharacters<&'a System> {
    // READ
    /// Read a single connective character by pair for any system
    pub fn read_connective_character_by_pair(&self, system_id: SystemId, pair: IndexPair) -> Option<&Connective> {
        self.system.connectives.get(&(system_id, pair))
    }

    /// Read all connective characters for a specific system as a vector
    pub fn read_connective_characters_as_vector(&self, system_id: SystemId) -> Vec<(IndexPair, &Connective)> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| if sid == &system_id { Some((*pair, conn)) } else { None })
            .collect()
    }

    /// Get all connective characters for a system as a HashMap
    pub fn read_connective_characters_as_map(&self, system_id: SystemId) -> std::collections::HashMap<IndexPair, &Connective> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| if sid == &system_id { Some((*pair, conn)) } else { None })
            .collect()
    }

    /// Get all connectives that involve a specific index for any system
    pub fn get_connectives_for_index(&self, system_id: SystemId, index: Index) -> Vec<(IndexPair, &Connective)> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| {
                if sid == &system_id && (pair.0 == index || pair.1 == index) {
                    Some((*pair, conn))
                } else {
                    None
                }
            })
            .collect()
    }

    // UTILITY
    /// Get the count of connectives for a specific system
    pub fn connective_count_for_system(&self, system_id: SystemId) -> usize {
        self.system.connectives.iter()
            .filter(|((sid, _), _)| sid == &system_id)
            .count()
    }

    /// Check if a connective exists at a specific pair for a system
    pub fn connective_exists(&self, system_id: SystemId, pair: IndexPair) -> bool {
        self.system.connectives.contains_key(&(system_id, pair))
    }

    /// Get all system IDs that have connectives
    pub fn get_systems_with_connectives(&self) -> std::collections::HashSet<SystemId> {
        self.system.connectives.keys()
            .map(|(sid, _)| sid.clone())
            .collect()
    }

    /// Get the degree (number of connections) for a specific index in a system
    pub fn get_index_degree(&self, system_id: SystemId, index: Index) -> usize {
        self.system.connectives.iter()
            .filter(|((sid, pair), _)| sid == &system_id && (pair.0 == index || pair.1 == index))
            .count()
    }
}

// Mutable implementation
impl<'a> ConnectiveCharacters<&'a mut System> {
    // CREATE
    /// Create a single connective character by indices for any system
    pub fn create_connective_character_by_indices(&mut self, system_id: SystemId, indices: (Index, Index), label: &str) {
        self.system.apply_event(StateEvent::CreateConnective {
            system_id,
            indices,
            character: label.to_string().to_lowercase(),
        });
    }

    /// Create multiple connective characters from a vector for any system
    pub fn create_connective_characters_by_vector(&mut self, system_id: SystemId, connectives: Vec<(IndexPair, String)>) {
        for (pair, character) in connectives {
            self.create_connective_character_by_indices(system_id.clone(), pair, &character);
        }
    }

    /// Load connective characters from a vocabulary implementation
    pub fn load_from_vocabulary<T: DefaultSystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        let connectives: Vec<_> = vocabulary.connective_characters()
            .iter()
            .map(|(connective_name, term1, term2)| {
                // Find the indices for the terms
                let term_chars = vocabulary.term_characters();
                let idx1 = term_chars.iter().position(|t| t == term1).unwrap_or(0) as Index;
                let idx2 = term_chars.iter().position(|t| t == term2).unwrap_or(0) as Index;
                let pair = if idx1 < idx2 { (idx1, idx2) } else { (idx2, idx1) };
                (pair, connective_name.to_string())
            })
            .collect();
        self.create_connective_characters_by_vector(system_id, connectives);
    }
    
    // READ
    /// Read a single connective character by pair for any system
    pub fn read_connective_character_by_pair(&self, system_id: SystemId, pair: IndexPair) -> Option<&Connective> {
        self.system.connectives.get(&(system_id, pair))
    }

    /// Read all connective characters for a specific system as a vector
    pub fn read_connective_characters_as_vector(&self, system_id: SystemId) -> Vec<(IndexPair, &Connective)> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| if sid == &system_id { Some((*pair, conn)) } else { None })
            .collect()
    }

    /// Get all connective characters for a system as a HashMap
    pub fn read_connective_characters_as_map(&self, system_id: SystemId) -> std::collections::HashMap<IndexPair, &Connective> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| if sid == &system_id { Some((*pair, conn)) } else { None })
            .collect()
    }

    /// Get all connectives that involve a specific index for any system
    pub fn get_connectives_for_index(&self, system_id: SystemId, index: Index) -> Vec<(IndexPair, &Connective)> {
        self.system.connectives.iter()
            .filter_map(|((sid, pair), conn)| {
                if sid == &system_id && (pair.0 == index || pair.1 == index) {
                    Some((*pair, conn))
                } else {
                    None
                }
            })
            .collect()
    }

    // UPDATE
    /// Update a single connective character by pair for any system
    pub fn update_connective_character_by_pair(&mut self, system_id: SystemId, pair: IndexPair, character: String) {
        self.system.apply_event(StateEvent::UpdateConnectiveCharacter { system_id, indices: pair, character });
    }

    /// Update multiple connective characters from a vector for any system
    pub fn update_connective_characters_by_vector(&mut self, system_id: SystemId, connectives: Vec<(IndexPair, String)>) {
        for (pair, character) in connectives {
            self.update_connective_character_by_pair(system_id.clone(), pair, character);
        }
    }

    // DELETE
    /// Delete a single connective by pair for any system
    pub fn delete_connective_by_pair(&mut self, system_id: SystemId, pair: IndexPair) {
        self.system.apply_event(StateEvent::DeleteConnective { system_id, indices: pair });
    }

    /// Delete multiple connectives by vector of pairs for any system
    pub fn delete_connectives_by_vector(&mut self, system_id: SystemId, pairs: Vec<IndexPair>) {
        for pair in pairs {
            self.delete_connective_by_pair(system_id.clone(), pair);
        }
    }

    /// Delete all connectives for a specific system
    pub fn delete_all_connectives_for_system(&mut self, system_id: SystemId) {
        let pairs: Vec<_> = self.system.connectives.keys()
            .filter_map(|(sid, pair)| if sid == &system_id { Some(*pair) } else { None })
            .collect();
        self.delete_connectives_by_vector(system_id, pairs);
    }

    /// Delete all connectives that involve a specific index for any system
    pub fn delete_connectives_for_index(&mut self, system_id: SystemId, index: Index) {
        let pairs: Vec<_> = self.system.connectives.keys()
            .filter_map(|(sid, pair)| {
                if sid == &system_id && (pair.0 == index || pair.1 == index) {
                    Some(*pair)
                } else {
                    None
                }
            })
            .collect();
        self.delete_connectives_by_vector(system_id, pairs);
    }

    // RESET
    /// Reset connectives to default values for any system
    pub fn reset_to_default(&mut self, system_id: SystemId, default_pairs: &[(IndexPair, &str)]) {
        let pairs: Vec<_> = self.system.connectives.keys()
            .filter_map(|(sid, pair)| if sid == &system_id { Some(*pair) } else { None })
            .collect();
        self.delete_connectives_by_vector(system_id.clone(), pairs);
        let connectives = default_pairs.iter().map(|(pair, name)| {
            (*pair, name.to_string())
        }).collect();
        self.create_connective_characters_by_vector(system_id, connectives);
    }

    /// Reset connectives to canonical vocabulary for any system
    pub fn reset_to_canonical<T: DefaultSystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.delete_all_connectives_for_system(system_id.clone());
        self.load_from_vocabulary(system_id, vocabulary);
    }

    // UTILITY (implement directly, don't delegate)
    /// Get the count of connectives for a specific system
    pub fn connective_count_for_system(&self, system_id: SystemId) -> usize {
        self.system.connectives.iter()
            .filter(|((sid, _), _)| sid == &system_id)
            .count()
    }

    /// Check if a connective exists at a specific pair for a system
    pub fn connective_exists(&self, system_id: SystemId, pair: IndexPair) -> bool {
        self.system.connectives.contains_key(&(system_id, pair))
    }

    /// Get all system IDs that have connectives
    pub fn get_systems_with_connectives(&self) -> std::collections::HashSet<SystemId> {
        self.system.connectives.keys()
            .map(|(sid, _)| sid.clone())
            .collect()
    }

    /// Get the degree (number of connections) for a specific index in a system
    pub fn get_index_degree(&self, system_id: SystemId, index: Index) -> usize {
        self.system.connectives.iter()
            .filter(|((sid, pair), _)| sid == &system_id && (pair.0 == index || pair.1 == index))
            .count()
    }
} 