## Architecture Overview

The API uses an event-driven architecture with dual manager implementations:

### Reading Operations
Use immutable references for concurrent access:
```rust
let system_ref = &system;
let term_manager = TermCharacters { system: system_ref };
let conn_manager = ConnectiveCharacters { system: system_ref };

// Can read from both simultaneously
let terms = term_manager.read_term_characters_as_vector(SystemId::Triad);
let connectives = conn_manager.read_connective_characters_as_vector(SystemId::Triad);
```

### Writing Operations  
Use mutable references through the event system:
```rust
let mut term_manager = TermCharacters { system: &mut system };
term_manager.create_term_character_by_index(SystemId::Triad, 0, "Will");
// All mutations go through StateEvent::CreateTerm, etc.
```

### State Management
All state is managed centrally by the `System` struct, with managers providing convenient views over the data. This gets its info from Generics, which get their info from the data files. 


---
### General direction

Horizon 1 API:
- State manager runs the operation in a hash table.
- Gets its info from genercis
- Which get their info from the data files

Horizon 2 API:
- Replace the hash table with a DHT / Content addressable storage
- Use hashs as field references
- Implement capabilities by reference

Horizon 3....
- Self describing protocols?
- Liberation tools?

TODO
- Rename designations?
- Spike capabilities?
- Spike nested systems? 