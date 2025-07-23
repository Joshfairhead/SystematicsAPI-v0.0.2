use crate::core::state_manager::{System, SystemId, Index, IndexPair};
use crate::core::traits::SystemData;
use super::{TermCharacters, ConnectiveCharacters, Designations};

/// High-level generic system manager that provides unified access to all CRUD operations.
/// This combines term characters, connective characters, and designations into a single interface.

pub struct SystemManager<'a> {
    pub system: &'a mut System,
}

impl<'a> SystemManager<'a> {
    /// Create a new system manager
    pub fn new(system: &'a mut System) -> Self {
        Self { system }
    }

    /// Get access to term character operations
    pub fn terms(&mut self) -> TermCharacters<&mut System> {
        TermCharacters { system: self.system }
    }

    /// Get access to connective character operations
    pub fn connectives(&mut self) -> ConnectiveCharacters<&mut System> {
        ConnectiveCharacters { system: self.system }
    }

    /// Get access to designation operations
    pub fn designations(&mut self) -> Designations<&mut System> {
        Designations { system: self.system }
    }

    // HIGH-LEVEL OPERATIONS
    /// Load a complete system from vocabulary (terms, connectives, and all designations)
    pub fn load_complete_system<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        // Load designations first
        self.designations().load_all_designations_from_vocabulary(system_id.clone(), vocabulary);
        
        // Load terms and connectives
        self.terms().load_from_vocabulary(system_id.clone(), vocabulary);
        self.connectives().load_from_vocabulary(system_id, vocabulary);
    }

    /// Delete a complete system (all terms, connectives, and designations)
    pub fn delete_complete_system(&mut self, system_id: SystemId) {
        self.terms().delete_all_terms_for_system(system_id.clone());
        self.connectives().delete_all_connectives_for_system(system_id.clone());
        self.designations().delete_all_designations_for_system(system_id);
    }

    /// Reset a complete system to canonical vocabulary
    pub fn reset_system_to_canonical<T: SystemData>(&mut self, system_id: SystemId, vocabulary: &T) {
        self.delete_complete_system(system_id.clone());
        self.load_complete_system(system_id, vocabulary);
    }

    /// Copy terms from one system to another
    pub fn copy_terms_between_systems(&mut self, from_system: SystemId, to_system: SystemId) {
        // Collect all terms first (owned)
        let terms: Vec<(Index, String)> = {
            let terms_ref = TermCharacters { system: &*self.system };
            terms_ref.read_term_characters_as_vector(from_system)
                .into_iter()
                .map(|(index, term)| (index, term.character.clone()))
                .collect()
        };
        for (index, character) in terms {
            self.terms().create_term_character_by_index(to_system.clone(), index, &character);
        }
    }

    /// Copy connectives from one system to another
    pub fn copy_connectives_between_systems(&mut self, from_system: SystemId, to_system: SystemId) {
        // Collect all connectives first (owned)
        let connectives: Vec<(IndexPair, String)> = {
            let conn_ref = ConnectiveCharacters { system: &*self.system };
            conn_ref.read_connective_characters_as_vector(from_system)
                .into_iter()
                .map(|(pair, conn)| (pair, conn.character.clone()))
                .collect()
        };
        for (pair, character) in connectives {
            self.connectives().create_connective_character_by_indices(to_system.clone(), pair, &character);
        }
    }

    /// Copy designations from one system to another
    pub fn copy_designations_between_systems(&mut self, from_system: SystemId, to_system: SystemId) {
        // Collect all designations first (owned)
        let (name, attr, term_des, conn_des, sources) = {
            let desig_ref = Designations { system: &*self.system };
            (
                desig_ref.read_system_name(from_system.clone()).cloned(),
                desig_ref.read_coherence_attribute(from_system.clone()).cloned(),
                desig_ref.read_term_designation(from_system.clone()).cloned(),
                desig_ref.read_connective_designation(from_system.clone()).cloned(),
                desig_ref.read_source_attribution(from_system).cloned(),
            )
        };
        if let Some(name) = name {
            self.designations().create_system_name(to_system.clone(), name);
        }
        if let Some(attr) = attr {
            self.designations().create_coherence_attribute(to_system.clone(), attr);
        }
        if let Some(term_des) = term_des {
            self.designations().create_term_designation(to_system.clone(), term_des);
        }
        if let Some(conn_des) = conn_des {
            self.designations().create_connective_designation(to_system.clone(), conn_des);
        }
        if let Some(sources) = sources {
            self.designations().create_source_attribution(to_system, sources);
        }
    }

    /// Copy an entire system to another system ID
    pub fn copy_system(&mut self, from_system: SystemId, to_system: SystemId) {
        self.copy_terms_between_systems(from_system.clone(), to_system.clone());
        self.copy_connectives_between_systems(from_system.clone(), to_system.clone());
        self.copy_designations_between_systems(from_system, to_system);
    }

    // SYSTEM ANALYSIS
    /// Get a complete summary of a system
    pub fn get_system_summary(&self, system_id: SystemId) -> SystemSummary {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let desig_ref = Designations { system: &*self.system };
        let term_count = terms_ref.term_count_for_system(system_id.clone());
        let connective_count = conn_ref.connective_count_for_system(system_id.clone());
        let designations = desig_ref.get_system_designation_summary(&system_id);
        SystemSummary {
            system_id: system_id.clone(),
            term_count,
            connective_count,
            designations,
        }
    }

    /// Compare two systems
    pub fn compare_systems(&self, system1: SystemId, system2: SystemId) -> SystemComparison {
        let summary1 = self.get_system_summary(system1.clone());
        let summary2 = self.get_system_summary(system2.clone());
        SystemComparison {
            system1: summary1.clone(),
            system2: summary2.clone(),
            term_count_difference: summary1.term_count as i32 - summary2.term_count as i32,
            connective_count_difference: summary1.connective_count as i32 - summary2.connective_count as i32,
        }
    }

    /// Get all systems in the state manager
    pub fn get_all_systems(&self) -> std::collections::HashSet<SystemId> {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let desig_ref = Designations { system: &*self.system };
        let mut systems = std::collections::HashSet::new();
        systems.extend(terms_ref.get_systems_with_terms());
        systems.extend(conn_ref.get_systems_with_connectives());
        systems.extend(desig_ref.get_systems_with_designations());
        systems
    }

    /// Check if a system exists (has any data)
    pub fn system_exists(&self, system_id: SystemId) -> bool {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let desig_ref = Designations { system: &*self.system };
        let term_count = terms_ref.term_count_for_system(system_id.clone());
        let conn_count = conn_ref.connective_count_for_system(system_id.clone());
        let has_desigs = desig_ref.system_has_designations(system_id);
        term_count > 0 || conn_count > 0 || has_desigs
    }

    // UTILITY OPERATIONS
    /// Get the degree distribution for a system
    pub fn get_system_degree_distribution(&self, system_id: SystemId) -> std::collections::HashMap<Index, usize> {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let term_count = terms_ref.term_count_for_system(system_id.clone());
        let mut distribution = std::collections::HashMap::new();
        for index in 0..term_count {
            let degree = conn_ref.get_index_degree(system_id.clone(), index as Index);
            distribution.insert(index as Index, degree);
        }
        distribution
    }

    /// Find isolated terms (terms with no connections) in a system
    pub fn find_isolated_terms(&self, system_id: SystemId) -> Vec<Index> {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let term_count = terms_ref.term_count_for_system(system_id.clone());
        let mut isolated = Vec::new();
        for index in 0..term_count {
            let degree = conn_ref.get_index_degree(system_id.clone(), index as Index);
            if degree == 0 {
                isolated.push(index as Index);
            }
        }
        isolated
    }

    /// Find hub terms (terms with many connections) in a system
    pub fn find_hub_terms(&self, system_id: SystemId, min_degree: usize) -> Vec<Index> {
        let terms_ref = TermCharacters { system: &*self.system };
        let conn_ref = ConnectiveCharacters { system: &*self.system };
        let term_count = terms_ref.term_count_for_system(system_id.clone());
        let mut hubs = Vec::new();
        for index in 0..term_count {
            let degree = conn_ref.get_index_degree(system_id.clone(), index as Index);
            if degree >= min_degree {
                hubs.push(index as Index);
            }
        }
        hubs
    }
}

/// Complete summary of a system
#[derive(Debug, Clone)]
pub struct SystemSummary {
    pub system_id: SystemId,
    pub term_count: usize,
    pub connective_count: usize,
    pub designations: super::designations::SystemDesignationSummary,
}

/// Comparison between two systems
#[derive(Debug, Clone)]
pub struct SystemComparison {
    pub system1: SystemSummary,
    pub system2: SystemSummary,
    pub term_count_difference: i32,
    pub connective_count_difference: i32,
} 