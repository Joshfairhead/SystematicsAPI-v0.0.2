use systematic_constructor::core::state_manager::*;

#[test]
fn test_default_octad_creation() {
    let mut system = System::new();
    system.default_system_octad();
    
    // Check all canonical indexes and coordinates exist
    assert_eq!(system.indexes.len(), 8);
    assert_eq!(system.coordinates.len(), 8);
    assert_eq!(system.index_pairs.len(), 28);
    
    // Verify specific coordinates
    let coord_0 = system.coordinates.get(&(SystemId::Octad, 0)).unwrap();
    assert_eq!(coord_0.x, 1.0);
    assert_eq!(coord_0.y, 0.0);
    assert_eq!(coord_0.z, None);
}

#[test]
fn test_coordinate_mutation() {
    let mut system = System::new();
    system.default_system_octad();
    
    let idx = 0;
    let new_coord = Coordinates { x: 2.0, y: 2.0, z: None };
    system.apply_event(StateEvent::UpdateCoordinates {
        system_id: SystemId::Octad,
        index: idx,
        coordinates: new_coord.clone(),
    });
    assert_eq!(system.coordinates.get(&(SystemId::Octad, idx)), Some(&new_coord));
}

#[test]
fn test_reset_to_canonical() {
    let mut system = System::new();
    system.default_system_octad();
    
    let idx = 0;
    let new_coord = Coordinates { x: 2.0, y: 2.0, z: None };
    system.apply_event(StateEvent::UpdateCoordinates {
        system_id: SystemId::Octad,
        index: idx,
        coordinates: new_coord,
    });
    
    // Reset using canonical data
    system.default_system_octad();
    
    // Verify reset worked
    let coord_0 = system.coordinates.get(&(SystemId::Octad, 0)).unwrap();
    assert_eq!(coord_0.x, 1.0);
    assert_eq!(coord_0.y, 0.0);
    assert_eq!(coord_0.z, None);
}

#[test]
fn test_event_driven_crud_terms_connectives_designations() {
    let mut system = System::new();
    system.default_system_octad();
    
    // Create a new term
    let idx = 8;
    let label = "extra".to_string();
    system.apply_event(StateEvent::CreateIndex { system_id: SystemId::Octad, index: idx });
    system.apply_event(StateEvent::CreateTerm { system_id: SystemId::Octad, index: idx, character: label.clone() });
    assert!(system.terms.get(&(SystemId::Octad, idx)).is_some());
    // Update term character
    let new_label = "updated".to_string();
    system.apply_event(StateEvent::UpdateTermCharacter { system_id: SystemId::Octad, index: idx, character: new_label.clone() });
    assert_eq!(&system.terms.get(&(SystemId::Octad, idx)).unwrap().character, &new_label);
    // Delete term
    system.apply_event(StateEvent::DeleteTerm { system_id: SystemId::Octad, index: idx });
    assert!(system.terms.get(&(SystemId::Octad, idx)).is_none());
    // Add a designation
    system.apply_event(StateEvent::CreateTermDesignation { system_id: SystemId::Octad, designation: "Element".to_string() });
    assert_eq!(system.term_designation.get(&SystemId::Octad), Some(&"Element".to_string()));
    // Update designation
    system.apply_event(StateEvent::UpdateTermDesignation { system_id: SystemId::Octad, designation: "UpdatedElement".to_string() });
    assert_eq!(system.term_designation.get(&SystemId::Octad), Some(&"UpdatedElement".to_string()));
    // Delete designation
    system.apply_event(StateEvent::DeleteTermDesignation { system_id: SystemId::Octad });
    assert!(system.term_designation.get(&SystemId::Octad).is_none());
}

#[test]
fn test_print_all_octad_data() {
    let mut system = System::new();
    system.default_system_octad();
    
    println!("\n=== COMPLETE OCTAD SYSTEM DATA ===");
    println!("=====================================");
    
    // System Overview
    println!("\n📊 SYSTEM OVERVIEW:");
    println!("  Total Indexes: {}", system.indexes.len());
    println!("  Total Coordinates: {}", system.coordinates.len());
    println!("  Total Terms: {}", system.terms.len());
    println!("  Total Connectives: {}", system.connectives.len());
    println!("  Total Index Pairs: {}", system.index_pairs.len());
    println!("  Total Term Designations: {}", system.term_designation.len());
    println!("  Total Connective Designations: {}", system.connective_designation.len());
    println!("  Total Coherence Attributes: {}", system.coherence_attributes.len());
    println!("  Total Source Attributions: {}", system.source_attributions.len());
    
    // System-Level Data (following mod.rs structure)
    println!("\n🏛️  SYSTEM-LEVEL DATA:");
    
    // System Name
    println!("  📛 System Name: Octad");
    
    // Coherence Attribute
    if let Some(attribute) = system.coherence_attributes.get(&SystemId::Octad) {
        println!("  🎯 Coherence Attribute: {:?}", attribute);
    }
    
    // Source Attribution
    if let Some(attribution) = system.source_attributions.get(&SystemId::Octad) {
        println!("  📚 Source Attribution: {:?}", attribution);
    }
    
    // Index-by-Index Data (following system structure)
    println!("\n🔢 INDEX-BY-INDEX DATA:");
    for index in 0..8 {
        println!("\n  📍 INDEX {}:", index);
        
        // Coordinates
        if let Some(coord) = system.coordinates.get(&(SystemId::Octad, index)) {
            println!("    🗺️  Coordinates: {:?}", coord);
        }
        
        // Term
        if let Some(term) = system.terms.get(&(SystemId::Octad, index)) {
            println!("    📝 Term: {:?}", term);
        }
        
        // Term Designation
        if let Some(designation) = system.term_designation.get(&SystemId::Octad) {
            println!("    🏷️  Term Designation: {:?}", designation);
        }
        
        // Connectives from this index
        println!("    🔗 Connectives:");
        for ((system_id, indices), connective) in system.connectives.iter() {
            if *system_id == SystemId::Octad && (indices.0 == index || indices.1 == index) {
                let other_index = if indices.0 == index { indices.1 } else { indices.0 };
                println!("      → {:?}: {:?} (to index {})", indices, connective.character, other_index);
            }
        }
    }
    
    // All Connectives Summary
    println!("\n🔗 ALL CONNECTIVES SUMMARY:");
    for ((system_id, indices), connective) in system.connectives.iter() {
        if *system_id == SystemId::Octad {
            println!("  {:?} → {:?}: {:?}", indices, connective.character, connective);
        }
    }
    
    // All Connective Designations
    println!("\n🏷️  ALL CONNECTIVE DESIGNATIONS:");
    if let Some(designation) = system.connective_designation.get(&SystemId::Octad) {
        println!("  System: {:?}", designation);
    }
    
    // Canonical Data Reference
    println!("\n📋 CANONICAL DATA REFERENCE:");
    println!("  Canonical Indexes: [0, 1, 2, 3, 4, 5, 6, 7]");
    println!("  Canonical Coordinates: [8 coordinates for octad system]");
    println!("  Canonical Edges: [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (3, 4), (3, 5), (3, 6), (3, 7), (4, 5), (4, 6), (4, 7), (5, 6), (5, 7), (6, 7)]");
    
    println!("\n=====================================");
    println!("=== END OCTAD SYSTEM DATA ===\n");
    
    // Verify we have the expected canonical data
    assert_eq!(system.indexes.len(), 8);
    assert_eq!(system.coordinates.len(), 8);
    assert_eq!(system.index_pairs.len(), 28);
}

#[test]
fn test_generic_mapper_with_triad() {
    use systematic_constructor::core::state_manager::{System, SystemId};
    use systematic_constructor::data::by_system::default_triad::DefaultTriadSystem;
    use systematic_constructor::core::traits::SystemData;
    
    let mut system = System::new();
    let vocabulary = DefaultTriadSystem::default();
    
    // Use the generic helper to load triad data
    system.load_canonical_data(SystemId::Triad, &vocabulary);
    
    println!("\n=== TRIAD SYSTEM USING GENERIC MAPPER ===");
    println!("System Name: {}", vocabulary.system_name());
    println!("Coherence Attribute: {}", vocabulary.coherence_attribute());
    println!("Term Designation: {:?}", vocabulary.term_designation());
    
    println!("\nTerms:");
    for (i, term) in vocabulary.term_characters().iter().enumerate() {
        println!("  Index {}: {:?}", i, term);
    }
    
    println!("\nConnectives:");
    for (i, connective) in vocabulary.connective_characters().iter().enumerate() {
        println!("  Index {}: {:?}", i, connective);
    }
    
    println!("\nSource Attributions:");
    for &source in vocabulary.source_attributions() {
        println!("  {}", source);
    }
    
    // Verify the data was loaded into the state manager
    assert_eq!(system.terms.len(), 3);
    assert_eq!(system.connectives.len(), 3);
    assert_eq!(system.coherence_attributes.len(), 1);
    assert_eq!(system.source_attributions.len(), 1);
    
    println!("\n=== END TRIAD SYSTEM ===");
} 

#[test]
fn test_octad_geometry_functionality() {
    println!("=== TESTING OCTAD GEOMETRY FUNCTIONALITY ===");
    
    let mut system = System::new();
    
    // Test the new convenience function
    system.default_system_octad();
    
    // Verify geometry was created
    assert_eq!(system.indexes.len(), 8);
    assert_eq!(system.coordinates.len(), 8);
    assert_eq!(system.index_pairs.len(), 28);
    
    // Verify specific coordinates
    let coord_0 = system.coordinates.get(&(SystemId::Octad, 0)).unwrap();
    assert_eq!(coord_0.x, 1.0);
    assert_eq!(coord_0.y, 0.0);
    assert_eq!(coord_0.z, None);
    
    let coord_1 = system.coordinates.get(&(SystemId::Octad, 1)).unwrap();
    assert!((coord_1.x - 0.70710678118).abs() < 0.0001);
    assert!((coord_1.y - (-0.70710678118)).abs() < 0.0001);
    
    // Verify edges were created
    assert!(system.index_pairs.contains(&(SystemId::Octad, (0, 1))));
    assert!(system.index_pairs.contains(&(SystemId::Octad, (0, 2))));
    assert!(system.index_pairs.contains(&(SystemId::Octad, (6, 7))));
    
    // Verify vocabulary data was also loaded
    assert_eq!(system.terms.len(), 8);
    assert_eq!(system.connectives.len(), 28);
    
    println!("✅ Octad geometry functionality works correctly");
    println!("  - Indexes: {}", system.indexes.len());
    println!("  - Coordinates: {}", system.coordinates.len());
    println!("  - Index pairs: {}", system.index_pairs.len());
    println!("  - Terms: {}", system.terms.len());
    println!("  - Connectives: {}", system.connectives.len());
} 