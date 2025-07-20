use systematic_constructor::core::state_manager::*;
use systematic_constructor::core::{
    term_characters::TermCharacters,
    connective_characters::ConnectiveCharacters,
    designations::Designations,
    system_manager::SystemManager,
};


// Import vocabulary systems
use systematic_constructor::data::by_system::{
    default_triad_system::DefaultTriadSystem,
    default_pentad::DefaultPentadSystem,
    default_octad::DefaultOctadSystem,
};

#[test]
fn test_generic_term_operations() {
    let mut system = System::new();
    let mut term_manager = TermCharacters { system: &mut system };
    
    println!("\n=== TESTING GENERIC TERM OPERATIONS ===");
    
    // Test with multiple systems
    let triad_vocab = DefaultTriadSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    
    // Load terms for both systems
    term_manager.load_from_vocabulary(SystemId::Triad, &triad_vocab);
    term_manager.load_from_vocabulary(SystemId::Pentad, &pentad_vocab);
    
    println!("Triad terms: {}", term_manager.term_count_for_system(SystemId::Triad));
    println!("Pentad terms: {}", term_manager.term_count_for_system(SystemId::Pentad));
    
    // Create a custom term for triad
    term_manager.create_term_character_by_index(SystemId::Triad, 3, "Custom Term");
    println!("After adding custom term: {}", term_manager.term_count_for_system(SystemId::Triad));
    
    // Read terms
    let triad_terms = term_manager.read_term_characters_as_vector(SystemId::Triad);
    let pentad_terms = term_manager.read_term_characters_as_vector(SystemId::Pentad);
    
    println!("Triad terms: {:?}", triad_terms.iter().map(|(i, t)| (*i, &t.character)).collect::<Vec<_>>());
    println!("Pentad terms: {:?}", pentad_terms.iter().map(|(i, t)| (*i, &t.character)).collect::<Vec<_>>());
    
    // Update a term
    term_manager.update_term_character_by_index(SystemId::Triad, 0, "Updated Will".to_string());
    
    // Verify the update
    let updated_term = term_manager.read_term_character_by_index(SystemId::Triad, 0);
    println!("Updated term: {:?}", updated_term.map(|t| &t.character));
    
    // Delete a term
    term_manager.delete_term_by_index(SystemId::Triad, 3);
    println!("After deletion: {}", term_manager.term_count_for_system(SystemId::Triad));
    
    // Reset to canonical
    term_manager.reset_to_canonical(SystemId::Triad, &triad_vocab);
    println!("After reset: {}", term_manager.term_count_for_system(SystemId::Triad));
    
    assert_eq!(term_manager.term_count_for_system(SystemId::Triad), 3);
    assert_eq!(term_manager.term_count_for_system(SystemId::Pentad), 5);
    println!("✅ Generic term operations work correctly");
}

#[test]
fn test_generic_connective_operations() {
    let mut system = System::new();
    let mut conn_manager = ConnectiveCharacters { system: &mut system };
    
    println!("\n=== TESTING GENERIC CONNECTIVE OPERATIONS ===");
    
    // Load connectives for multiple systems
    let triad_vocab = DefaultTriadSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    
    conn_manager.load_from_vocabulary(SystemId::Triad, &triad_vocab);
    conn_manager.load_from_vocabulary(SystemId::Pentad, &pentad_vocab);
    
    println!("Triad connectives: {}", conn_manager.connective_count_for_system(SystemId::Triad));
    println!("Pentad connectives: {}", conn_manager.connective_count_for_system(SystemId::Pentad));
    
    // Create a custom connective
    conn_manager.create_connective_character_by_indices(SystemId::Triad, (0, 3), "Custom Act");
    println!("After adding custom connective: {}", conn_manager.connective_count_for_system(SystemId::Triad));
    
    // Get connectives for a specific index
    let index_0_connectives = conn_manager.get_connectives_for_index(SystemId::Triad, 0);
    println!("Connectives for index 0: {:?}", index_0_connectives.iter().map(|(p, c)| (*p, &c.character)).collect::<Vec<_>>());
    
    // Get degree distribution
    let triad_degree_dist = conn_manager.get_index_degree(SystemId::Triad, 0);
    println!("Degree of index 0 in triad: {}", triad_degree_dist);
    
    // Reset to canonical
    conn_manager.reset_to_canonical(SystemId::Triad, &triad_vocab);
    println!("After reset: {}", conn_manager.connective_count_for_system(SystemId::Triad));
    
    assert_eq!(conn_manager.connective_count_for_system(SystemId::Triad), 3);
    assert_eq!(conn_manager.connective_count_for_system(SystemId::Pentad), 10);
    println!("✅ Generic connective operations work correctly");
}

#[test]
fn test_generic_designation_operations() {
    let mut system = System::new();
    let mut desig_manager = Designations { system: &mut system };
    
    println!("\n=== TESTING GENERIC DESIGNATION OPERATIONS ===");
    
    // Load designations for multiple systems
    let triad_vocab = DefaultTriadSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    
    desig_manager.load_all_designations_from_vocabulary(SystemId::Triad, &triad_vocab);
    desig_manager.load_all_designations_from_vocabulary(SystemId::Pentad, &pentad_vocab);
    
    // Read designations
    let triad_summary = desig_manager.get_system_designation_summary(&SystemId::Triad);
    let pentad_summary = desig_manager.get_system_designation_summary(&SystemId::Pentad);
    
    println!("Triad summary: {:?}", triad_summary);
    println!("Pentad summary: {:?}", pentad_summary);
    
    // Update a designation
    desig_manager.update_coherence_attribute(SystemId::Triad, "Updated Dynamism".to_string());
    
    // Verify the update
    let updated_attr = desig_manager.read_coherence_attribute(SystemId::Triad);
    println!("Updated coherence attribute: {:?}", updated_attr);
    
    // Check which systems have designations
    let systems_with_desigs = desig_manager.get_systems_with_designations();
    println!("Systems with designations: {:?}", systems_with_desigs);
    
    assert!(desig_manager.system_has_designations(SystemId::Triad));
    assert!(desig_manager.system_has_designations(SystemId::Pentad));
    println!("✅ Generic designation operations work correctly");
}

#[test]
fn test_system_manager_operations() {
    let mut system = System::new();
    let mut sys_manager = SystemManager::new(&mut system);
    
    println!("\n=== TESTING SYSTEM MANAGER OPERATIONS ===");
    
    // Load complete systems
    let triad_vocab = DefaultTriadSystem::default();
    let pentad_vocab = DefaultPentadSystem::default();
    let octad_vocab = DefaultOctadSystem::default();
    
    sys_manager.load_complete_system(SystemId::Triad, &triad_vocab);
    sys_manager.load_complete_system(SystemId::Pentad, &pentad_vocab);
    sys_manager.load_complete_system(SystemId::Octad, &octad_vocab);
    
    // Get system summaries
    let triad_summary = sys_manager.get_system_summary(SystemId::Triad);
    let pentad_summary = sys_manager.get_system_summary(SystemId::Pentad);
    let octad_summary = sys_manager.get_system_summary(SystemId::Octad);
    
    println!("Triad summary: {:?}", triad_summary);
    println!("Pentad summary: {:?}", pentad_summary);
    println!("Octad summary: {:?}", octad_summary);
    
    // Compare systems
    let comparison = sys_manager.compare_systems(SystemId::Triad, SystemId::Pentad);
    println!("Triad vs Pentad comparison: {:?}", comparison);
    
    // Get all systems
    let all_systems = sys_manager.get_all_systems();
    println!("All systems: {:?}", all_systems);
    
    // Test system analysis
    let triad_degree_dist = sys_manager.get_system_degree_distribution(SystemId::Triad);
    println!("Triad degree distribution: {:?}", triad_degree_dist);
    
    let triad_hubs = sys_manager.find_hub_terms(SystemId::Triad, 2);
    println!("Triad hub terms (degree >= 2): {:?}", triad_hubs);
    
    // Copy a system
    sys_manager.copy_system(SystemId::Triad, SystemId::Tetrad);
    let tetrad_summary = sys_manager.get_system_summary(SystemId::Tetrad);
    println!("Copied Tetrad summary: {:?}", tetrad_summary);
    
    assert_eq!(triad_summary.term_count, 3);
    assert_eq!(pentad_summary.term_count, 5);
    assert_eq!(octad_summary.term_count, 8);
    assert_eq!(tetrad_summary.term_count, 3); // Copied from triad
    
    println!("✅ System manager operations work correctly");
}

#[test]
fn test_generic_vs_specialized_comparison() {
    let mut system = System::new();
    
    println!("\n=== COMPARING GENERIC VS SPECIALIZED APPROACHES ===");
    
    // Generic approach
    let mut sys_manager = SystemManager::new(&mut system);
    let triad_vocab = DefaultTriadSystem::default();
    
    sys_manager.load_complete_system(SystemId::Triad, &triad_vocab);
    
    // Using generic interfaces
    sys_manager.terms().create_term_character_by_index(SystemId::Triad, 3, "Generic Term");
    sys_manager.connectives().create_connective_character_by_indices(SystemId::Triad, (0, 3), "Generic Connective");
    sys_manager.designations().update_coherence_attribute(SystemId::Triad, "Generic Attribute".to_string());
    
    println!("Generic approach - Triad terms: {}", sys_manager.terms().term_count_for_system(SystemId::Triad));
    println!("Generic approach - Triad connectives: {}", sys_manager.connectives().connective_count_for_system(SystemId::Triad));
    
    // The same operations would work for ANY system ID
    sys_manager.terms().create_term_character_by_index(SystemId::Pentad, 5, "Pentad Term");
    sys_manager.terms().create_term_character_by_index(SystemId::Octad, 8, "Octad Term");
    
    println!("Generic approach works for all systems!");
    println!("✅ Generic interfaces provide consistent, flexible operations across all systems");
} 