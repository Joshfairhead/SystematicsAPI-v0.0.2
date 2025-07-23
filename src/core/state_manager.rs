// Generic, reusable topology/state logic for all systems

use std::collections::{HashMap, HashSet};

use crate::core::term_characters::Term;
use crate::core::connective_characters::Connective;

use crate::core::traits::{SystemData, GeometryData};
// SourceAttribution is not present in by_component, so we will define a placeholder type for now

type SourceAttribution = Vec<String>; // TODO: Replace with a more specific type if available

// Add Coordinate struct
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>, // None for 2D, Some(z) for 3D
}

// Type aliases for clarity
pub type Index = usize;
pub type IndexPair = (Index, Index);
pub type CoordinatePair = (Coordinates, Coordinates);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemId {
    Monad,
    Dyad,
    Triad,
    Tetrad,
    Pentad,
    Hexad,
    Heptad,
    Octad,
    Nonad,
    Decad,
    Undecad,
    Duodecad,
    Custom(String),
}

pub type TermKey = (SystemId, Index);
pub type ConnectiveKey = (SystemId, IndexPair);

// Full event enum for generic state
pub enum StateEvent {
    //---------------------------
    // Topological Operations
    //---------------------------
    
    // Create
    CreateIndex { system_id: SystemId, index: Index },
    CreateIndexPair { system_id: SystemId, indices: IndexPair },
    
    // Read (for completeness, these are not events but included for API symmetry)
    ReadIndex { system_id: SystemId, index: Index },
    ReadIndexPair { system_id: SystemId, indices: IndexPair },

    // Update
    UpdateIndex { system_id: SystemId, old_index: Index, new_index: Index },
    UpdateIndexPair { system_id: SystemId, old_indices: IndexPair, new_indices: IndexPair },

    // Delete
    DeleteIndex { system_id: SystemId, index: Index },
    DeleteIndexPair { system_id: SystemId, indices: IndexPair },

    //---------------------------
    // Geometry Operations
    //---------------------------
    
    // Create
    CreateCoordinates { system_id: SystemId, index: Index, coordinates: Coordinates },
    CreateCoordinatePair { system_id: SystemId, indices: IndexPair, coordinates: CoordinatePair },
    
    // Read
    ReadCoordinates { system_id: SystemId, index: Index },
    ReadCoordinatePair { system_id: SystemId, indices: IndexPair },
    
    // Update
    UpdateCoordinates { system_id: SystemId, index: Index, coordinates: Coordinates },
    UpdateCoordinatePair { system_id: SystemId, indices: IndexPair, coordinates: CoordinatePair },
    
    // Delete
    DeleteCoordinates { system_id: SystemId, index: Index },
    DeleteCoordinatePair { system_id: SystemId, indices: IndexPair },

    //---------------------------
    // Semantic Operations
    //---------------------------
    
    // Create
    CreateTerm { system_id: SystemId, index: Index, character: String },
    CreateConnective { system_id: SystemId, indices: IndexPair, character: String },
    
    // Read (no-op for event log, but included for API symmetry)
    ReadTerm { system_id: SystemId, index: Index },
    ReadConnective { system_id: SystemId, indices: IndexPair },
    
    // Update
    UpdateTermCharacter { system_id: SystemId, index: Index, character: String },
    UpdateConnectiveCharacter { system_id: SystemId, indices: IndexPair, character: String },
    
    // Delete
    DeleteTerm { system_id: SystemId, index: Index },
    DeleteConnective { system_id: SystemId, indices: IndexPair },

    //---------------------------
    // System Attribute Operations
    //---------------------------
    // Create
    CreateSystemName { system_id: SystemId, name: String },
    CreateCoherenceAttribute { system_id: SystemId, attribute: String },
    CreateTermDesignation { system_id: SystemId, designation: String },
    CreateConnectiveDesignation { system_id: SystemId, designation: String },
    CreateSourceAttribution { system_id: SystemId, source: SourceAttribution },
    
    // Read (no-op for event log, but included for API symmetry)
    ReadSystemName { system_id: SystemId },
    ReadCoherenceAttribute { system_id: SystemId },
    ReadTermDesignation { system_id: SystemId },
    ReadConnectiveDesignation { system_id: SystemId },
    ReadSourceAttribution { system_id: SystemId },
    
    // Update
    UpdateSystemName { system_id: SystemId, name: String },
    UpdateCoherenceAttribute { system_id: SystemId, attribute: String },
    UpdateTermDesignation { system_id: SystemId, designation: String },
    UpdateConnectiveDesignation { system_id: SystemId, designation: String },
    UpdateSourceAttribution { system_id: SystemId, source: SourceAttribution },
    
    // Delete
    DeleteSystemName { system_id: SystemId },
    DeleteCoherenceAttribute { system_id: SystemId },
    DeleteTermDesignation { system_id: SystemId },
    DeleteConnectiveDesignation { system_id: SystemId },
    DeleteSourceAttribution { system_id: SystemId },
}

// State manager struct
pub struct System {
    // Topological
    pub indexes: HashSet<(SystemId, Index)>,
    pub index_pairs: HashSet<(SystemId, IndexPair)>,
    
    // Geometry
    pub coordinates: HashMap<(SystemId, Index), Coordinates>,
    pub coordinate_pairs: HashMap<(SystemId, IndexPair), CoordinatePair>,
    
    // Semantic
    pub coherence_attributes: HashMap<SystemId, String>,
    pub term_designation: HashMap<SystemId, String>,
    pub terms: HashMap<(SystemId, Index), Term>,
    pub connective_designation: HashMap<SystemId, String>,
    pub connectives: HashMap<(SystemId, IndexPair), Connective>,
    pub source_attributions: HashMap<SystemId, SourceAttribution>,
    pub system_names: HashMap<SystemId, String>, // System name for each system    
}

impl System {
    pub fn new() -> Self {
        Self {
            // Topological
            indexes: HashSet::new(),
            index_pairs: HashSet::new(),
            // - Laplacian matrix computation
            // - Spectral analysis
            // - Graph neural network operations

            // Geometry
            coordinates: HashMap::new(),
            coordinate_pairs: HashMap::new(),
            // Semantic
            term_designation: HashMap::new(),
            terms: HashMap::new(),
            connective_designation: HashMap::new(),
            connectives: HashMap::new(),
            // Coherence attribute for each system
            coherence_attributes: HashMap::new(),
            // Source attribution for each system (placeholder type)
            source_attributions: HashMap::new(),
            system_names: HashMap::new(),
        }
    }

    // Granular Event handler
    pub fn apply_event(&mut self, event: StateEvent) {
        match event {
            //---------------------------
            // Topological Operations
            //---------------------------

            StateEvent::CreateIndex { system_id, index } => { self.indexes.insert((system_id, index)); }
            StateEvent::CreateIndexPair { system_id, indices } => { self.index_pairs.insert((system_id, indices)); }
            StateEvent::ReadIndex { .. } => {}
            StateEvent::ReadIndexPair { .. } => {}
            StateEvent::UpdateIndex { system_id, old_index, new_index } => {
                // Remove old index and add new index
                self.indexes.remove(&(system_id.clone(), old_index));
                self.indexes.insert((system_id.clone(), new_index));
                // Update coordinates
                if let Some(coord) = self.coordinates.remove(&(system_id.clone(), old_index)) {
                    self.coordinates.insert((system_id.clone(), new_index), coord);
                }
                // Update terms
                if let Some(term) = self.terms.remove(&(system_id.clone(), old_index)) {
                    self.terms.insert((system_id.clone(), new_index), term);
                }
                // Update all index pairs and related data
                let mut updated_pairs = vec![];
                for (sid, pair) in self.index_pairs.clone().into_iter() {
                    if sid == system_id && (pair.0 == old_index || pair.1 == old_index) {
                        let new_pair = (
                            if pair.0 == old_index { new_index } else { pair.0 },
                            if pair.1 == old_index { new_index } else { pair.1 },
                        );
                        self.index_pairs.remove(&(sid.clone(), pair));
                        self.index_pairs.insert((sid.clone(), new_pair));
                        // Update coordinate_pairs
                        if let Some(coord_pair) = self.coordinate_pairs.remove(&(sid.clone(), pair)) {
                            self.coordinate_pairs.insert((sid.clone(), new_pair), coord_pair);
                        }
                        // Update connectives
                        if let Some(conn) = self.connectives.remove(&(sid.clone(), pair)) {
                            self.connectives.insert((sid.clone(), new_pair), conn);
                        }
                        updated_pairs.push((sid, pair, new_pair));
                    }
                }
            }
            StateEvent::UpdateIndexPair { system_id, old_indices, new_indices } => {
                // Remove old index pair and add new index pair
                self.index_pairs.remove(&(system_id.clone(), old_indices));
                self.index_pairs.insert((system_id.clone(), new_indices));
                // Update coordinate_pairs
                if let Some(coord_pair) = self.coordinate_pairs.remove(&(system_id.clone(), old_indices)) {
                    self.coordinate_pairs.insert((system_id.clone(), new_indices), coord_pair);
                }
                // Update connectives
                if let Some(conn) = self.connectives.remove(&(system_id.clone(), old_indices)) {
                    self.connectives.insert((system_id.clone(), new_indices), conn);
                }
            }
            StateEvent::DeleteIndex { ref system_id, index } => {
                self.indexes.remove(&(system_id.clone(), index));
                self.coordinates.remove(&(system_id.clone(), index));
                self.terms.remove(&(system_id.clone(), index));
                self.index_pairs.retain(|&(ref sid, pair)| sid != system_id || pair.1 != index);
                self.coordinate_pairs.retain(|&(ref sid, pair), _| sid != system_id || pair.1 != index);
                self.connectives.retain(|&(ref sid, pair), _| sid != system_id || pair.1 != index);
            }
            StateEvent::DeleteIndexPair { ref system_id, indices } => {
                self.index_pairs.remove(&(system_id.clone(), indices));
                self.coordinate_pairs.remove(&(system_id.clone(), indices));
                self.connectives.remove(&(system_id.clone(), indices));
            }

            //---------------------------
            // Geometry Operations
            //---------------------------

            StateEvent::CreateCoordinates { system_id, index, coordinates } => { self.coordinates.insert((system_id, index), coordinates); }
            StateEvent::CreateCoordinatePair { system_id, indices, coordinates } => { self.coordinate_pairs.insert((system_id, indices), coordinates); }
            StateEvent::ReadCoordinates { .. } => {}
            StateEvent::ReadCoordinatePair { .. } => {}
            StateEvent::UpdateCoordinates { ref system_id, index, coordinates } => {
                if self.indexes.contains(&(system_id.clone(), index)) {
                    self.coordinates.insert((system_id.clone(), index), coordinates);
                }
            }
            StateEvent::UpdateCoordinatePair { ref system_id, indices, coordinates } => {
                if self.index_pairs.contains(&(system_id.clone(), indices)) {
                    self.coordinate_pairs.insert((system_id.clone(), indices), coordinates);
                }
            }
            StateEvent::DeleteCoordinates { system_id, index } => { self.coordinates.remove(&(system_id, index)); }
            StateEvent::DeleteCoordinatePair { system_id, indices } => { self.coordinate_pairs.remove(&(system_id, indices)); }

            //---------------------------
            // Semantic Operations
            //---------------------------

            StateEvent::CreateTerm { system_id, index, character } => {
                self.terms.insert((system_id, index), Term { index, character });
            }
            StateEvent::CreateConnective { system_id, indices, character } => {
                self.connectives.insert((system_id, indices), Connective { indices, character });
            }
            StateEvent::ReadTerm { .. } => {}
            StateEvent::ReadConnective { .. } => {}
            StateEvent::UpdateTermCharacter { system_id, index, character } => {
                if let Some(term) = self.terms.get_mut(&(system_id, index)) {
                    term.character = character;
                }
            }
            StateEvent::UpdateConnectiveCharacter { system_id, indices, character } => {
                if let Some(conn) = self.connectives.get_mut(&(system_id, indices)) {
                    conn.character = character;
                }
            }
            StateEvent::DeleteTerm { system_id, index } => { self.terms.remove(&(system_id, index)); }
            StateEvent::DeleteConnective { system_id, indices } => { self.connectives.remove(&(system_id, indices)); }

            //---------------------------
            // System Attribute Operations
            //---------------------------
            // Create
            StateEvent::CreateSystemName { system_id, name } => {
                self.system_names.insert(system_id, name);
            }
            StateEvent::CreateCoherenceAttribute { system_id, attribute } => {
                self.coherence_attributes.insert(system_id, attribute);
            }
            StateEvent::CreateTermDesignation { system_id, designation } => {
                self.term_designation.insert(system_id, designation);
            }
            StateEvent::CreateConnectiveDesignation { system_id, designation } => {
                self.connective_designation.insert(system_id, designation);
            }
            StateEvent::CreateSourceAttribution { system_id, source } => {
                self.source_attributions.insert(system_id, source);
            }
            
            // Read (no-op for event log, but included for API symmetry)
            StateEvent::ReadSystemName { system_id: _ } => {}
            StateEvent::ReadCoherenceAttribute { system_id: _ } => {}
            StateEvent::ReadTermDesignation { system_id: _ } => {}
            StateEvent::ReadConnectiveDesignation { system_id: _ } => {}
            StateEvent::ReadSourceAttribution { system_id: _ } => {}
            
            // Update
            StateEvent::UpdateSystemName { system_id, name } => {
                self.system_names.insert(system_id, name);
            }
            StateEvent::UpdateCoherenceAttribute { system_id, attribute } => {
                self.coherence_attributes.insert(system_id, attribute);
            }
            StateEvent::UpdateTermDesignation { system_id, designation } => {
                self.term_designation.insert(system_id, designation);
            }
            StateEvent::UpdateConnectiveDesignation { system_id, designation } => {
                self.connective_designation.insert(system_id, designation);
            }
            StateEvent::UpdateSourceAttribution { system_id, source } => {
                self.source_attributions.insert(system_id, source);
            }
            
            // Delete
            StateEvent::DeleteSystemName { system_id } => {
                self.system_names.remove(&system_id);
            }
            StateEvent::DeleteCoherenceAttribute { system_id } => {
                self.coherence_attributes.remove(&system_id);
            }
            StateEvent::DeleteTermDesignation { system_id } => {
                self.term_designation.remove(&system_id);
            }
            StateEvent::DeleteConnectiveDesignation { system_id } => {
                self.connective_designation.remove(&system_id);
            }
            StateEvent::DeleteSourceAttribution { system_id } => {
                self.source_attributions.remove(&system_id);
            }
        }
        // Possibly composite event handlers
    }
}

// Normalize an index pair (for undirected edges)
pub fn normalize_pair(a: Index, b: Index) -> IndexPair {
    if a < b { (a, b) } else { (b, a) }
}

//---------------------------
// HELPER METHODS TO LOAD CANONICAL DATA
//---------------------------

impl System {
    /// Load canonical semantic data for a system
    pub fn load_canonical_data<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        self.load_canonical_attributes(system_id.clone(), system_data);
        self.load_canonical_term_designation(system_id.clone(), system_data);
        self.load_canonical_terms(system_id.clone(), system_data);
        self.load_canonical_connective_designation(system_id.clone(), system_data);
        self.load_canonical_connectives(system_id.clone(), system_data);
        self.load_canonical_sources(system_id.clone(), system_data);
    }

    fn load_canonical_attributes<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        self.apply_event(StateEvent::CreateSystemName { 
            system_id: system_id.clone(), 
            name: system_data.system_name().to_string() 
        });
        self.apply_event(StateEvent::CreateCoherenceAttribute { 
            system_id, 
            attribute: system_data.coherence_attribute().to_string() 
        });
    }

    fn load_canonical_term_designation<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        self.apply_event(StateEvent::CreateTermDesignation { 
            system_id, 
            designation: system_data.term_designation().to_string() 
        });
    }

    fn load_canonical_terms<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        for (i, character) in system_data.term_characters().iter().enumerate() {
            self.apply_event(StateEvent::CreateTerm { 
                system_id: system_id.clone(), 
                index: i, 
                character: character.to_string() 
            });
        }
    }

    fn load_canonical_connective_designation<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        self.apply_event(StateEvent::CreateConnectiveDesignation { 
            system_id, 
            designation: system_data.connective_designation().to_string() 
        });
    }

    fn load_canonical_connectives<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        for (_i, (connective_name, term1, term2)) in system_data.connective_characters().iter().enumerate() {
            let pair = normalize_pair(
                system_data.term_characters().iter().position(|&t| t == *term1).unwrap(),
                system_data.term_characters().iter().position(|&t| t == *term2).unwrap()
            );
            self.apply_event(StateEvent::CreateConnective { 
                system_id: system_id.clone(), 
                indices: pair, 
                character: connective_name.to_string() 
            });
        }
    }

    fn load_canonical_sources<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        let sources: SourceAttribution = system_data.source_attributions().iter().map(|s| s.to_string()).collect();
        self.apply_event(StateEvent::CreateSourceAttribution { 
            system_id, 
            source: sources 
        });
    }

    //--------------------------------
    // Builders
    //--------------------------------

    /// Load complete system data including geometry (if available)
    pub fn load_complete_system<SD: SystemData>(&mut self, system_id: SystemId, system_data: &SD) {
        // Load semantic data
        self.load_canonical_data(system_id.clone(), system_data);
        
        // Build geometry if the system has geometry data
        if !system_data.indexes().is_empty() {
            // Use SystemData's geometry methods directly
            for (i, coord) in system_data.indexes().iter().zip(system_data.coordinates().iter()) {
                self.apply_event(StateEvent::CreateIndex { 
                    system_id: system_id.clone(), 
                    index: *i 
                });
                self.apply_event(StateEvent::CreateCoordinates { 
                    system_id: system_id.clone(), 
                    index: *i, 
                    coordinates: coord.clone() 
                });
            }
            
            // Create edges
            for &(i, j) in system_data.edges().iter() {
                let pair = normalize_pair(i, j);
                self.apply_event(StateEvent::CreateIndexPair { 
                    system_id: system_id.clone(), 
                    indices: pair 
                });
            }
        }
    }

    /// Build geometry from pure geometry data (no semantic content)
    pub fn build_pure_geometry<G: GeometryData>(&mut self, system_id: SystemId, geometry: &G) {
        // Create nodes and assign coordinates
        for (i, coord) in geometry.indexes().iter().zip(geometry.coordinates().iter()) {
            self.apply_event(StateEvent::CreateIndex { 
                system_id: system_id.clone(), 
                index: *i 
            });
            self.apply_event(StateEvent::CreateCoordinates { 
                system_id: system_id.clone(), 
                index: *i, 
                coordinates: coord.clone() 
            });
        }
        
        // Create edges
        for &(i, j) in geometry.edges().iter() {
            let pair = normalize_pair(i, j);
            self.apply_event(StateEvent::CreateIndexPair { 
                system_id: system_id.clone(), 
                indices: pair 
            });
        }
    }

    //---------------------------
    // CONVENIENCE FUNCTIONS FOR DEFAULT SYSTEMS
    //---------------------------

    /// Create a default monad system
    pub fn default_system_monad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::MonadSemantics;
        use crate::data::by_geometry::K1Geometry;
        
        let semantics = MonadSemantics;
        let geometry = K1Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Monad, &system_data);
    }

    /// Create a default dyad system
    pub fn default_system_dyad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::DyadSemantics;
        use crate::data::by_geometry::K2Geometry;
        
        let semantics = DyadSemantics;
        let geometry = K2Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Dyad, &system_data);
    }

    /// Create a default triad system
    pub fn default_system_triad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::TriadSemantics;
        use crate::data::by_geometry::K3Geometry;
        
        let semantics = TriadSemantics;
        let geometry = K3Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Triad, &system_data);
    }

    /// Create a default tetrad system
    pub fn default_system_tetrad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::TetradSemantics;
        use crate::data::by_geometry::K4Geometry;
        
        let semantics = TetradSemantics;
        let geometry = K4Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Tetrad, &system_data);
    }

    /// Create a default pentad system
    pub fn default_system_pentad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::PentadSemantics;
        use crate::data::by_geometry::K5Geometry;
        
        let semantics = PentadSemantics;
        let geometry = K5Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Pentad, &system_data);
    }

    /// Create a default hexad system
    pub fn default_system_hexad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::HexadSemantics;
        use crate::data::by_geometry::K6Geometry;
        
        let semantics = HexadSemantics;
        let geometry = K6Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Hexad, &system_data);
    }

    /// Create a default heptad system
    pub fn default_system_heptad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::HeptadSemantics;
        use crate::data::by_geometry::K7Geometry;
        
        let semantics = HeptadSemantics;
        let geometry = K7Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Heptad, &system_data);
    }

    /// Create a default octad system with full geometry
    pub fn default_system_octad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::OctadSemantics;
        use crate::data::by_geometry::K8Geometry;
        
        let semantics = OctadSemantics;
        let geometry = K8Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Octad, &system_data);
    }

    /// Create a default ennead system
    pub fn default_system_ennead(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::EnneadSemantics;
        use crate::data::by_geometry::K9Geometry;
        
        let semantics = EnneadSemantics;
        let geometry = K9Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Nonad, &system_data);
    }

    /// Create a default decad system
    pub fn default_system_decad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::DecadSemantics;
        use crate::data::by_geometry::K10Geometry;
        
        let semantics = DecadSemantics;
        let geometry = K10Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Decad, &system_data);
    }

    /// Create a default undecad system
    pub fn default_system_undecad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::UndecadSemantics;
        use crate::data::by_geometry::K11Geometry;
        
        let semantics = UndecadSemantics;
        let geometry = K11Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Undecad, &system_data);
    }

    /// Create a default dodecad system
    pub fn default_system_dodecad(&mut self) {
        use crate::core::adapters::SystemDataAdapter;
        use crate::data::by_semantics::DodecadSemantics;
        use crate::data::by_geometry::K12Geometry;
        
        let semantics = DodecadSemantics;
        let geometry = K12Geometry;
        let system_data = SystemDataAdapter::new(semantics, geometry);
        self.load_complete_system(SystemId::Duodecad, &system_data);
    }

    //---------------------------
    // PURE GEOMETRY CONVENIENCE FUNCTIONS
    //---------------------------

    /// Build a pure K1 graph (no semantic content)
    pub fn pure_k1_graph(&mut self) {
        use crate::data::by_geometry::K1Geometry;
        
        let geometry = K1Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK1".to_string()), &geometry);
    }

    /// Build a pure K2 graph (no semantic content)
    pub fn pure_k2_graph(&mut self) {
        use crate::data::by_geometry::K2Geometry;
        
        let geometry = K2Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK2".to_string()), &geometry);
    }

    /// Build a pure K3 graph (no semantic content)
    pub fn pure_k3_graph(&mut self) {
        use crate::data::by_geometry::K3Geometry;
        
        let geometry = K3Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK3".to_string()), &geometry);
    }

    /// Build a pure K4 graph (no semantic content)
    pub fn pure_k4_graph(&mut self) {
        use crate::data::by_geometry::K4Geometry;
        
        let geometry = K4Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK4".to_string()), &geometry);
    }

    /// Build a pure K5 graph (no semantic content)
    pub fn pure_k5_graph(&mut self) {
        use crate::data::by_geometry::K5Geometry;
        
        let geometry = K5Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK5".to_string()), &geometry);
    }

    /// Build a pure K6 graph (no semantic content)
    pub fn pure_k6_graph(&mut self) {
        use crate::data::by_geometry::K6Geometry;
        
        let geometry = K6Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK6".to_string()), &geometry);
    }

    /// Build a pure K7 graph (no semantic content)
    pub fn pure_k7_graph(&mut self) {
        use crate::data::by_geometry::K7Geometry;
        
        let geometry = K7Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK7".to_string()), &geometry);
    }

    /// Build a pure K8 graph (no semantic content)
    pub fn pure_k8_graph(&mut self) {
        use crate::data::by_geometry::K8Geometry;
        
        let geometry = K8Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK8".to_string()), &geometry);
    }

    /// Build a pure K9 graph (no semantic content)
    pub fn pure_k9_graph(&mut self) {
        use crate::data::by_geometry::K9Geometry;
        
        let geometry = K9Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK9".to_string()), &geometry);
    }

    /// Build a pure K10 graph (no semantic content)
    pub fn pure_k10_graph(&mut self) {
        use crate::data::by_geometry::K10Geometry;
        
        let geometry = K10Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK10".to_string()), &geometry);
    }

    /// Build a pure K11 graph (no semantic content)
    pub fn pure_k11_graph(&mut self) {
        use crate::data::by_geometry::K11Geometry;
        
        let geometry = K11Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK11".to_string()), &geometry);
    }

    /// Build a pure K12 graph (no semantic content)
    pub fn pure_k12_graph(&mut self) {
        use crate::data::by_geometry::K12Geometry;
        
        let geometry = K12Geometry;
        self.build_pure_geometry(SystemId::Custom("PureK12".to_string()), &geometry);
    }
}
