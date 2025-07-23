use systematic_constructor::core::state_manager::{System, SystemId};

#[test]
fn test_basic_system_operations() {
    let mut system = System::new();
    
    println!("\n=== BASIC SYSTEM OPERATIONS DEMONSTRATION ===");
    
    // 1. CREATE CANONICAL TETRAD
    println!("\n1. Creating canonical tetrad...");
    system.default_system_tetrad();
    
    // 2. VERIFY THE SYSTEM WAS CREATED
    println!("\n2. Verifying the system was created...");
    
    // Access fields directly
    let term_count = system.terms.len();
    let coordinate_count = system.coordinates.len();
    let index_pair_count = system.index_pairs.len();
    
    println!("Total terms: {}", term_count);
    println!("Total coordinates: {}", coordinate_count);
    println!("Total index pairs: {}", index_pair_count);
    
    // Check if tetrad system name was set
    let tetrad_name = system.system_names.get(&SystemId::Tetrad);
    println!("Tetrad system name: {:?}", tetrad_name);
    
    assert!(term_count > 0);
    assert!(coordinate_count > 0);
    assert!(index_pair_count > 0);
    
    println!("✅ Basic system operations work correctly");
} 