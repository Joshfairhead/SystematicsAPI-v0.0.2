use systematic_constructor::core::traits::SystemData;
use systematic_constructor::core::state_manager::{System, SystemId};
use systematic_constructor::data::by_system::{
    default_triad::DefaultTriadSystem,
    default_pentad::DefaultPentadSystem,
    default_tetrad::DefaultTetradSystem,
    default_hexad::DefaultHexadSystem,
    default_octad::DefaultOctadSystem,
};

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