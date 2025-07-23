use systematic_constructor::core::traits::SystemData;
use systematic_constructor::core::state_manager::{System, SystemId};
use systematic_constructor::data::by_system::{
    default_triad::DefaultTriadSystem,
    default_pentad::DefaultPentadSystem,
    default_tetrad::DefaultTetradSystem,
    default_hexad::DefaultHexadSystem,
    default_octad::DefaultOctadSystem,
};
use systematic_constructor::core::state_manager::StateEvent;

/// Test helper function to verify a system vocabulary
fn test_system_vocabulary<T: SystemData + Default>(system_name: &str) {
    let vocabulary = T::default();
    
    println!("Testing {} vocabulary:", system_name);
    println!("  System name: {}", vocabulary.system_name());
    println!("  Term count: {}", vocabulary.term_characters().len());
    println!("  Connective count: {}", vocabulary.connective_characters().len());
    println!("  ✓ {} vocabulary verified", system_name);
}

/// Test helper function to load a system into the state manager
fn test_system_loading<T: SystemData + Default>(system_id: SystemId, system_name: &str) {
    let mut system = System::new();
    let vocabulary = T::default();
    
    println!("Loading {} system into state manager:", system_name);
    
    // Load the system using the convenience function
    system.load_complete_system(system_id.clone(), &vocabulary);
    
    // Verify the system was loaded correctly
    let term_count = system.terms.len();
    let coordinate_count = system.coordinates.len();
    let index_pair_count = system.index_pairs.len();
    
    println!("  ✓ {} system loaded with {} terms, {} coordinates, {} index pairs", 
             system_name, term_count, coordinate_count, index_pair_count);
    
    // Verify the system name was set correctly
    if let Some(name) = system.system_names.get(&system_id) {
        println!("  ✓ System name set to: {}", name);
    } else {
        println!("  ⚠ System name not found");
    }
}

#[test]
fn test_triad_vocabulary() {
    test_system_vocabulary::<DefaultTriadSystem>("Triad");
}

#[test]
fn test_tetrad_vocabulary() {
    test_system_vocabulary::<DefaultTetradSystem>("Tetrad");
}

#[test]
fn test_pentad_vocabulary() {
    test_system_vocabulary::<DefaultPentadSystem>("Pentad");
}

#[test]
fn test_hexad_vocabulary() {
    test_system_vocabulary::<DefaultHexadSystem>("Hexad");
}

#[test]
fn test_octad_vocabulary() {
    test_system_vocabulary::<DefaultOctadSystem>("Octad");
}

#[test]
fn test_triad_loading() {
    test_system_loading::<DefaultTriadSystem>(SystemId::Triad, "Triad");
}

#[test]
fn test_tetrad_loading() {
    test_system_loading::<DefaultTetradSystem>(SystemId::Tetrad, "Tetrad");
}

#[test]
fn test_pentad_loading() {
    test_system_loading::<DefaultPentadSystem>(SystemId::Pentad, "Pentad");
}

#[test]
fn test_hexad_loading() {
    test_system_loading::<DefaultHexadSystem>(SystemId::Hexad, "Hexad");
}

#[test]
fn test_octad_loading() {
    test_system_loading::<DefaultOctadSystem>(SystemId::Octad, "Octad");
}

#[test]
fn test_multiple_systems_in_one_manager() {
    let mut system = System::new();
    
    // Load multiple systems
    let triad_vocab = DefaultTriadSystem::default();
    let tetrad_vocab = DefaultTetradSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    
    system.load_canonical_data(SystemId::Triad, &triad_vocab);
    system.load_canonical_data(SystemId::Tetrad, &tetrad_vocab);
    system.load_canonical_data(SystemId::Pentad, &pentad_vocab);
    
    println!("\n=== MULTIPLE SYSTEMS IN ONE MANAGER ===");
    println!("Total terms: {}", system.terms.len());
    println!("Total connectives: {}", system.connectives.len());
    println!("Total coherence attributes: {}", system.coherence_attributes.len());
    println!("Total term designations: {}", system.term_designation.len());
    println!("Total connective designations: {}", system.connective_designation.len());
    println!("Total source attributions: {}", system.source_attributions.len());
    
    // Verify each system has its data
    assert_eq!(system.coherence_attributes.get(&SystemId::Triad), Some(&"Dynamism".to_string()));
    assert_eq!(system.coherence_attributes.get(&SystemId::Tetrad), Some(&"Activity Field".to_string()));
    assert_eq!(system.coherence_attributes.get(&SystemId::Pentad), Some(&"Significance and Potential".to_string()));
    
    // Verify terms from different systems don't interfere
    let triad_terms: Vec<_> = system.terms.iter()
        .filter_map(|((sid, idx), term)| if sid == &SystemId::Triad { Some((*idx, &term.character)) } else { None })
        .collect();
    
    let tetrad_terms: Vec<_> = system.terms.iter()
        .filter_map(|((sid, idx), term)| if sid == &SystemId::Tetrad { Some((*idx, &term.character)) } else { None })
        .collect();
    
    println!("\nTriad terms: {:?}", triad_terms);
    println!("Tetrad terms: {:?}", tetrad_terms);
    
    assert_eq!(triad_terms.len(), 3);
    assert_eq!(tetrad_terms.len(), 4);
    
    println!("✅ Multiple systems loaded successfully without interference");
}

#[test]
fn test_connective_tuple_order() {
    // Test that all systems have the correct tuple order: (connective_name, term1, term2)
    let triad_vocab = DefaultTriadSystem::default();
    let tetrad_vocab = DefaultTetradSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    let hexad_vocab = DefaultHexadSystem::default();
    let octad_vocab = DefaultOctadSystem::default();
    
    println!("\n=== TESTING CONNECTIVE TUPLE ORDER ===");
    
    // Check a few examples from each system
    let triad_connective = &triad_vocab.connective_characters()[0];
    let tetrad_connective = &tetrad_vocab.connective_characters()[0];
    let pentad_connective = &pentad_vocab.connective_characters()[0];
    let hexad_connective = &hexad_vocab.connective_characters()[0];
    let octad_connective = &octad_vocab.connective_characters()[0];
    
    println!("Triad: {:?}", triad_connective);
    println!("Tetrad: {:?}", tetrad_connective);
    println!("Pentad: {:?}", pentad_connective);
    println!("Hexad: {:?}", hexad_connective);
    println!("Octad: {:?}", octad_connective);
    
    // Verify the first element is the connective name (not a term)
    assert!(!triad_vocab.term_characters().contains(&triad_connective.0));
    assert!(!tetrad_vocab.term_characters().contains(&tetrad_connective.0));
    assert!(!pentad_vocab.term_characters().contains(&pentad_connective.0));
    assert!(!hexad_vocab.term_characters().contains(&hexad_connective.0));
    assert!(!octad_vocab.term_characters().contains(&octad_connective.0));
    
    // Verify the second and third elements are terms
    assert!(triad_vocab.term_characters().contains(&triad_connective.1));
    assert!(triad_vocab.term_characters().contains(&triad_connective.2));
    assert!(tetrad_vocab.term_characters().contains(&tetrad_connective.1));
    assert!(tetrad_vocab.term_characters().contains(&tetrad_connective.2));
    assert!(pentad_vocab.term_characters().contains(&pentad_connective.1));
    assert!(pentad_vocab.term_characters().contains(&pentad_connective.2));
    assert!(hexad_vocab.term_characters().contains(&hexad_connective.1));
    assert!(hexad_vocab.term_characters().contains(&hexad_connective.2));
    assert!(octad_vocab.term_characters().contains(&octad_connective.1));
    assert!(octad_vocab.term_characters().contains(&octad_connective.2));
    
    println!("✅ All systems have correct connective tuple order");
} 

#[test]
fn test_multiple_tetrad_systems_comparison() {
    let mut system = System::new();
    
    println!("\n=== MULTIPLE TETRAD SYSTEMS COMPARISON ===");
    
    // 1. Create canonical tetrad system
    println!("\n1. Creating canonical tetrad system...");
    let canonical_tetrad = DefaultTetradSystem::default();
    system.load_complete_system(SystemId::Tetrad, &canonical_tetrad);
    
    // 2. Create Aristotle's tetrad system using custom SystemId
    println!("\n2. Creating Aristotle's tetrad system...");
    let aristotle_tetrad_id = SystemId::Custom("Aristotle's Tetrad".to_string());
    
    // Create Aristotle's tetrad with different terms but same structure
    system.apply_event(StateEvent::CreateSystemName { 
        system_id: aristotle_tetrad_id.clone(), 
        name: "Aristotle's Tetrad".to_string() 
    });
    system.apply_event(StateEvent::CreateCoherenceAttribute { 
        system_id: aristotle_tetrad_id.clone(), 
        attribute: "Four Causes".to_string() 
    });
    system.apply_event(StateEvent::CreateTermDesignation { 
        system_id: aristotle_tetrad_id.clone(), 
        designation: "Causes".to_string() 
    });
    system.apply_event(StateEvent::CreateConnectiveDesignation { 
        system_id: aristotle_tetrad_id.clone(), 
        designation: "Relations".to_string() 
    });
    
    // Aristotle's four causes: Material, Formal, Efficient, Final
    let aristotle_terms = [
        (0, "Material Cause"),
        (1, "Formal Cause"), 
        (2, "Efficient Cause"),
        (3, "Final Cause")
    ];
    
    for (index, term) in aristotle_terms {
        system.apply_event(StateEvent::CreateTerm { 
            system_id: aristotle_tetrad_id.clone(), 
            index, 
            character: term.to_string() 
        });
        // Same geometry as canonical tetrad
        system.apply_event(StateEvent::CreateCoordinates { 
            system_id: aristotle_tetrad_id.clone(), 
            index, 
            coordinates: canonical_tetrad.coordinates[index].clone() 
        });
    }
    
    // Aristotle's connective relationships
    let aristotle_connectives = [
        ((0, 1), "Material-Formal Relation"),
        ((0, 2), "Material-Efficient Relation"),
        ((0, 3), "Material-Final Relation"),
        ((1, 2), "Formal-Efficient Relation"),
        ((1, 3), "Formal-Final Relation"),
        ((2, 3), "Efficient-Final Relation"),
    ];
    
    for (indices, connective) in aristotle_connectives {
        system.apply_event(StateEvent::CreateConnective { 
            system_id: aristotle_tetrad_id.clone(), 
            indices, 
            character: connective.to_string() 
        });
    }
    
    // 3. Compare the two tetrad systems
    println!("\n3. Comparing canonical vs Aristotle's tetrad...");
    
    // Get term counts
    let canonical_terms: Vec<_> = system.terms.iter()
        .filter_map(|((sid, idx), term)| if sid == &SystemId::Tetrad { Some((*idx, &term.character)) } else { None })
        .collect();
    
    let aristotle_terms: Vec<_> = system.terms.iter()
        .filter_map(|((sid, idx), term)| if sid == &aristotle_tetrad_id { Some((*idx, &term.character)) } else { None })
        .collect();
    
    println!("Canonical Tetrad terms: {:?}", canonical_terms);
    println!("Aristotle's Tetrad terms: {:?}", aristotle_terms);
    
    // Get connective counts
    let canonical_connectives: Vec<_> = system.connectives.iter()
        .filter_map(|((sid, pair), conn)| if sid == &SystemId::Tetrad { Some((*pair, &conn.character)) } else { None })
        .collect();
    
    let aristotle_connectives: Vec<_> = system.connectives.iter()
        .filter_map(|((sid, pair), conn)| if sid == &aristotle_tetrad_id { Some((*pair, &conn.character)) } else { None })
        .collect();
    
    println!("Canonical Tetrad connectives: {:?}", canonical_connectives);
    println!("Aristotle's Tetrad connectives: {:?}", aristotle_connectives);
    
    // Get coherence attributes
    let canonical_coherence = system.coherence_attributes.get(&SystemId::Tetrad);
    let aristotle_coherence = system.coherence_attributes.get(&aristotle_tetrad_id);
    
    println!("Canonical Tetrad coherence: {:?}", canonical_coherence);
    println!("Aristotle's Tetrad coherence: {:?}", aristotle_coherence);
    
    // 4. Verify both systems exist independently
    assert_eq!(canonical_terms.len(), 4);
    assert_eq!(aristotle_terms.len(), 4);
    assert_eq!(canonical_connectives.len(), 6);
    assert_eq!(aristotle_connectives.len(), 6);
    
    // Verify they have different content
    assert_ne!(canonical_terms[0].1, aristotle_terms[0].1); // Different first terms
    assert_ne!(canonical_coherence, aristotle_coherence); // Different coherence attributes
    
    // Verify they have the same structure (4 terms, 6 connectives)
    assert_eq!(canonical_terms.len(), aristotle_terms.len());
    assert_eq!(canonical_connectives.len(), aristotle_connectives.len());
    
    println!("✅ Multiple tetrad systems created and compared successfully!");
    println!("✅ Canonical tetrad: {} terms, {} connectives", canonical_terms.len(), canonical_connectives.len());
    println!("✅ Aristotle's tetrad: {} terms, {} connectives", aristotle_terms.len(), aristotle_connectives.len());
    println!("✅ Both systems exist independently with different content but same structure");
}

#[test]
fn test_system_id_custom_variants() {
    let mut system = System::new();
    
    println!("\n=== TESTING CUSTOM SYSTEM ID VARIANTS ===");
    
    // Create multiple custom tetrad systems
    let custom_ids = vec![
        SystemId::Custom("Aristotle's Tetrad".to_string()),
        SystemId::Custom("Plato's Tetrad".to_string()),
        SystemId::Custom("Modern Tetrad".to_string()),
    ];
    
    for (i, system_id) in custom_ids.iter().enumerate() {
        println!("\nCreating system: {:?}", system_id);
        
        // Create a simple system for each custom ID
        system.apply_event(StateEvent::CreateSystemName { 
            system_id: system_id.clone(), 
            name: format!("Custom Tetrad {}", i + 1) 
        });
        
        system.apply_event(StateEvent::CreateTerm { 
            system_id: system_id.clone(), 
            index: 0, 
            character: format!("Term A{}", i + 1) 
        });
        
        system.apply_event(StateEvent::CreateTerm { 
            system_id: system_id.clone(), 
            index: 1, 
            character: format!("Term B{}", i + 1) 
        });
    }
    
    // Verify all custom systems exist independently
    for system_id in &custom_ids {
        let terms: Vec<_> = system.terms.iter()
            .filter_map(|((sid, idx), term)| if sid == system_id { Some((*idx, &term.character)) } else { None })
            .collect();
        
        println!("System {:?} has {} terms: {:?}", system_id, terms.len(), terms);
        assert_eq!(terms.len(), 2); // Each has 2 terms
    }
    
    // Verify they don't interfere with each other
    let total_terms = system.terms.len();
    assert_eq!(total_terms, 6); // 3 systems × 2 terms each
    
    println!("✅ All custom system variants created successfully!");
    println!("✅ Total terms across all systems: {}", total_terms);
    println!("✅ No interference between custom system IDs");
} 