use systematic_constructor::core::state_manager::*;
use systematic_constructor::core::system_manager::SystemManager;

#[test]
fn test_system_operations_demo() {
    let mut system = System::new();
    let mut sys_manager = SystemManager::new(&mut system);
    
    println!("\n=== SYSTEM OPERATIONS DEMONSTRATION ===");
    
    // 1. CREATE CANONICAL TETRAD
    println!("\n1. Creating canonical tetrad...");
    sys_manager.system.default_system_tetrad();
    
    // 2. CREATE ARISTOTLE'S FOUR CAUSES TETRAD
    println!("\n2. Creating Aristotle's Four Causes tetrad...");
    let aristotle_tetrad = SystemId::Custom("AristotleFourCauses".to_string());
    
    // Create Aristotle's four causes as terms
    sys_manager.terms().create_term_character_by_index(aristotle_tetrad.clone(), 0, "Material Cause");
    sys_manager.terms().create_term_character_by_index(aristotle_tetrad.clone(), 1, "Formal Cause");
    sys_manager.terms().create_term_character_by_index(aristotle_tetrad.clone(), 2, "Efficient Cause");
    sys_manager.terms().create_term_character_by_index(aristotle_tetrad.clone(), 3, "Final Cause");
    
    // Create connectives between the causes
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (0, 1), "Material-Form Relation");
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (1, 2), "Form-Efficiency Relation");
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (2, 3), "Efficiency-Final Relation");
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (0, 2), "Material-Efficiency Relation");
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (1, 3), "Form-Final Relation");
    sys_manager.connectives().create_connective_character_by_indices(aristotle_tetrad.clone(), (0, 3), "Material-Final Relation");
    
    // Set Aristotle's system metadata
    sys_manager.designations().create_system_name(aristotle_tetrad.clone(), "Aristotle's Four Causes".to_string());
    sys_manager.designations().create_coherence_attribute(aristotle_tetrad.clone(), "Causal Explanation".to_string());
    sys_manager.designations().create_term_designation(aristotle_tetrad.clone(), "Causes".to_string());
    sys_manager.designations().create_connective_designation(aristotle_tetrad.clone(), "Relations".to_string());
    sys_manager.designations().create_source_attribution(aristotle_tetrad.clone(), vec!["Aristotle's Physics".to_string(), "Metaphysics".to_string()]);
    
    // 3. COMPARE THE TWO TETRADS
    println!("\n3. Comparing the two tetrads...");
    let comparison = sys_manager.compare_systems(SystemId::Tetrad, aristotle_tetrad.clone());
    println!("Comparison: {:?}", comparison);
    
    // 4. NEST ONE SYSTEM INSIDE ANOTHER (as a term)
    println!("\n4. Nesting Aristotle's system as a term in the canonical tetrad...");
    
    // Create a new system that represents the canonical tetrad with Aristotle's system nested
    let nested_tetrad = SystemId::Custom("NestedTetrad".to_string());
    
    // Copy the canonical tetrad structure
    sys_manager.copy_system(SystemId::Tetrad, nested_tetrad.clone());
    
    // Replace one term with a reference to Aristotle's system
    sys_manager.terms().update_term_character_by_index(nested_tetrad.clone(), 0, "Aristotle's Four Causes System".to_string());
    
    // Add a new connective that represents the relationship to the nested system
    sys_manager.connectives().create_connective_character_by_indices(nested_tetrad.clone(), (0, 1), "Causal Analysis of Ideal");
    sys_manager.connectives().create_connective_character_by_indices(nested_tetrad.clone(), (0, 2), "Causal Analysis of Directive");
    sys_manager.connectives().create_connective_character_by_indices(nested_tetrad.clone(), (0, 3), "Causal Analysis of Ground");
    
    // Update the system metadata
    sys_manager.designations().update_system_name(nested_tetrad.clone(), "Tetrad with Nested Aristotle System".to_string());
    sys_manager.designations().update_coherence_attribute(nested_tetrad.clone(), "Nested Causal Analysis".to_string());
    
    // 5. DEMONSTRATE CAPABILITY INJECTION THROUGH HASH TABLE REFERENCES
    println!("\n5. Demonstrating capability injection through hash table references...");
    
    // Create a capability system
    let capability_system = SystemId::Custom("Capabilities".to_string());
    
    // Define capabilities as terms
    sys_manager.terms().create_term_character_by_index(capability_system.clone(), 0, "Analytical Capability");
    sys_manager.terms().create_term_character_by_index(capability_system.clone(), 1, "Synthetic Capability");
    sys_manager.terms().create_term_character_by_index(capability_system.clone(), 2, "Transformative Capability");
    
    // Create capability relationships
    sys_manager.connectives().create_connective_character_by_indices(capability_system.clone(), (0, 1), "Analysis-Synthesis");
    sys_manager.connectives().create_connective_character_by_indices(capability_system.clone(), (1, 2), "Synthesis-Transformation");
    sys_manager.connectives().create_connective_character_by_indices(capability_system.clone(), (0, 2), "Analysis-Transformation");
    
    // Associate capabilities with Aristotle's system through metadata
    sys_manager.designations().create_source_attribution(capability_system.clone(), vec!["Aristotle's Four Causes".to_string()]);
    
    // 6. DEMONSTRATE ARBITRARY FIELD ACCESS
    println!("\n6. Demonstrating arbitrary field access...");
    
    // Show that we can access any field by SystemId
    let desig_ref_for_read = sys_manager.designations();
    let canonical_tetrad_name = desig_ref_for_read.read_system_name(SystemId::Tetrad);
    let aristotle_name = desig_ref_for_read.read_system_name(aristotle_tetrad.clone());
    let nested_name = desig_ref_for_read.read_system_name(nested_tetrad.clone());
    
    println!("Canonical Tetrad name: {:?}", canonical_tetrad_name);
    println!("Aristotle system name: {:?}", aristotle_name);
    println!("Nested system name: {:?}", nested_name);
    
    // Show we can access any field with arbitrary data
    let all_systems = sys_manager.get_all_systems();
    println!("All systems in hash table: {:?}", all_systems);
    
    // 7. DEMONSTRATE SYSTEM ANALYSIS
    println!("\n7. Demonstrating system analysis capabilities...");
    
    for system_id in all_systems {
        let summary = sys_manager.get_system_summary(system_id.clone());
        println!("System {:?}: {} terms, {} connectives", system_id, summary.term_count, summary.connective_count);
        
        if summary.term_count > 0 {
            let degree_dist = sys_manager.get_system_degree_distribution(system_id.clone());
            println!("  Degree distribution: {:?}", degree_dist);
            
            let hubs = sys_manager.find_hub_terms(system_id.clone(), 2);
            println!("  Hub terms (degree >= 2): {:?}", hubs);
        }
    }
    
    println!("\n✅ All system operations demonstrated successfully!");
}

#[test]
fn test_arbitrary_field_access() {
    let mut system = System::new();
    let mut sys_manager = SystemManager::new(&mut system);
    
    println!("\n=== ARBITRARY FIELD ACCESS DEMONSTRATION ===");
    
    // Create a custom system with arbitrary data
    let custom_system = SystemId::Custom("MyCustomSystem".to_string());
    
    // We can store ANY string data in ANY field
    sys_manager.designations().create_system_name(custom_system.clone(), "Whatever I want".to_string());
    sys_manager.designations().create_coherence_attribute(custom_system.clone(), "My arbitrary attribute".to_string());
    sys_manager.designations().create_term_designation(custom_system.clone(), "My custom term type".to_string());
    sys_manager.designations().create_connective_designation(custom_system.clone(), "My custom connective type".to_string());
    sys_manager.designations().create_source_attribution(custom_system.clone(), vec!["My source".to_string(), "Another source".to_string()]);
    
    // We can access any field by SystemId - the keys are SystemId, values are arbitrary strings/vectors
    let desig_ref = sys_manager.designations();
    let name = desig_ref.read_system_name(custom_system.clone());
    let attr = desig_ref.read_coherence_attribute(custom_system.clone());
    let term_des = desig_ref.read_term_designation(custom_system.clone());
    let conn_des = desig_ref.read_connective_designation(custom_system.clone());
    let sources = desig_ref.read_source_attribution(custom_system.clone());
    
    println!("Custom system data:");
    println!("  Name: {:?}", name);
    println!("  Attribute: {:?}", attr);
    println!("  Term designation: {:?}", term_des);
    println!("  Connective designation: {:?}", conn_des);
    println!("  Sources: {:?}", sources);
    
    // We can update any field with new arbitrary data
    drop(desig_ref); // Release the read-only borrow
    sys_manager.designations().update_system_name(custom_system.clone(), "Updated name".to_string());
    sys_manager.designations().update_coherence_attribute(custom_system.clone(), "Updated attribute".to_string());
    
    let updated_desig_ref = sys_manager.designations();
    let updated_name = updated_desig_ref.read_system_name(custom_system.clone());
    let updated_attr = updated_desig_ref.read_coherence_attribute(custom_system.clone());
    
    println!("After updates:");
    println!("  Name: {:?}", updated_name);
    println!("  Attribute: {:?}", updated_attr);
    
    println!("\n✅ Arbitrary field access demonstrated!");
} 