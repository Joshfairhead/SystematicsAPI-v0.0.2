## Architecture Overview

The API uses an event-driven architecture with dual manager implementations:

### Core Architecture

The system is built around three main traits:
- **`SystemData`**: Complete system interface combining semantics and geometry
- **`SemanticData`**: Pure semantic content (terms, connectives, designations)
- **`GeometryData`**: Pure geometric structure (coordinates, edges)

### Adapter Pattern

Systems are implemented using the `SystemDataAdapter` which combines semantic and geometry data:

```rust
use systematic_constructor::core::adapters::SystemDataAdapter;
use systematic_constructor::data::by_semantics::TriadSemantics;
use systematic_constructor::data::by_geometry::K3Geometry;

let triad_system = SystemDataAdapter {
    semantics: TriadSemantics,
    geometry: K3Geometry,
};
```

### Dual Manager Implementation

The system provides dual manager implementations for read-only and mutable operations:

#### Reading Operations
Use immutable references for concurrent access:
```rust
let system_ref = &system;
let term_manager = TermCharacters { system: system_ref };
let conn_manager = ConnectiveCharacters { system: system_ref };

// Can read from both simultaneously
let terms = term_manager.read_term_characters_as_vector(SystemId::Triad);
let connectives = conn_manager.read_connective_characters_as_vector(SystemId::Triad);
```

#### Writing Operations  
Use mutable references through the event system:
```rust
let mut term_manager = TermCharacters { system: &mut system };
let mut conn_manager = ConnectiveCharacters { system: &mut system };

term_manager.create_term_character_by_index(SystemId::Triad, 0, "Will");
conn_manager.create_connective_character_by_indices(SystemId::Triad, (0, 1), "Will-Being");
// All mutations go through StateEvent::CreateTerm, etc.
```

### State Management

All state is managed by the `System` struct with event-driven mutations:

```rust
let mut system = System::new();

// Load complete system
let triad_vocab = DefaultTriadSystem::default();
system.load_complete_system(SystemId::Triad, &triad_vocab);

// Apply events for mutations
system.apply_event(StateEvent::CreateTerm {
    system_id: SystemId::Triad,
    index: 3,
    character: "New Term".to_string(),
});
```

### Direct Field Access

You can also access data directly through public fields:

```rust
// Read terms
let term_count = system.terms.len();
let triad_terms: Vec<_> = system.terms.iter()
    .filter(|((sid, _), _)| sid == &SystemId::Triad)
    .collect();

// Read coordinates
let coordinate_count = system.coordinates.len();

// Read system names
let triad_name = system.system_names.get(&SystemId::Triad);
```

### Convenience Functions

Pre-built systems can be loaded using convenience functions:

```rust
// Load canonical systems
system.default_system_triad();
system.default_system_tetrad();
system.default_system_pentad();

// Load pure geometry
system.pure_k3_graph();
system.pure_k5_graph();
```

---
### General Direction

**Horizon 1 API** (Current):
- Event-driven state management with hash table storage
- Dual manager implementation for read/write operations
- Trait-based system data with adapter pattern
- Separated semantic and geometry data

**Horizon 2 API** (Planned):
- Replace hash table with DHT / Content addressable storage
- Use hashes as field references
- Implement capabilities by reference

**Horizon 3** (Future):
- Self describing protocols
- Liberation tools

### TODO
- [ ] Add more convenience functions for system operations
- [ ] Implement system comparison and analysis tools
- [ ] Add serialization/deserialization support
- [ ] Spike capabilities implementation
- [ ] Spike nested systems support
- [ ] Spike Laplacian Positional Encoding
- [ ] Spike Spectral Attention Networks 



