# Persistence and Library Feature Plan

## Overview

This document outlines the persistence and library functionality for the Systematics API, enabling users to save, load, and manage system data in multiple formats. The architecture supports both snapshot-based persistence (like the current `by_system` files) and event-driven persistence for audit trails and undo/redo functionality.

## Current State

### Existing Infrastructure
- ✅ **Event-driven architecture** with `StateEvent` enum
- ✅ **System data structures** ready for serialization
- ✅ **Dual manager pattern** for read-only and mutable operations
- ✅ **Static `by_system` files** as reference format
- ❌ **No serialization dependencies** (serde, etc.)
- ❌ **No persistence methods** implemented

### Current `by_system` Files as Templates
The existing `src/data/by_system/` files serve as the reference format for system data snapshots:
- `default_triad.rs` - Complete triad system definition
- `default_tetrad.rs` - Complete tetrad system definition
- `default_pentad.rs` - Complete pentad system definition
- etc.

## Feature Plan: Two Persistence Formats

### Format 1: System Data Snapshot

**Purpose**: Complete system snapshots for loading predefined systems, sharing definitions, and human-readable persistence.

**Use Cases**:
- Loading canonical systems (triad, tetrad, etc.)
- Sharing custom system definitions
- Version control of system configurations
- Human editing of system data
- Fast system reconstruction

**Format**: JSON/TOML with semantic + geometric data

**Example Structure**:
```json
{
  "name": "Triad",
  "coherence_attribute": "Dynamism",
  "term_designation": "Impulses",
  "connective_designation": "Acts",
  "term_characters": ["Will", "Function", "Being"],
  "connective_characters": [
    ["Act1", "Will", "Function"],
    ["Act2", "Function", "Being"],
    ["Act3", "Being", "Will"]
  ],
  "source_attributions": ["Elementary Systematics", "Bennett's Work"],
  "geometry": {
    "indexes": [0, 1, 2],
    "coordinates": [
      {"x": 0.0, "y": 1.0, "z": null},
      {"x": 0.0, "y": -1.0, "z": null},
      {"x": 1.0, "y": 0.0, "z": null}
    ],
    "edges": [[0, 1], [1, 2], [2, 0]]
  }
}
```

**Implementation Requirements**:
1. Add serde dependencies to `Cargo.toml`
2. Make data structures serializable (`#[derive(Serialize, Deserialize)]`)
3. Convert `&'static str` to `String` for serialization
4. Implement save/load methods on `System`
5. Support both JSON and TOML formats

### Format 2: Event Log

**Purpose**: Replayable sequence of operations for audit trails, undo/redo, and incremental persistence.

**Use Cases**:
- Audit trails of system modifications
- Undo/redo functionality
- Debugging system state changes
- Collaborative editing (merge event streams)
- Incremental persistence
- State reconstruction from operations

**Format**: JSON array of timestamped events

**Example Structure**:
```json
[
  {
    "timestamp": "2024-01-15T10:30:00Z",
    "event": {
      "type": "CreateSystemName",
      "system_id": "Triad",
      "name": "Triad"
    }
  },
  {
    "timestamp": "2024-01-15T10:30:01Z",
    "event": {
      "type": "CreateCoherenceAttribute",
      "system_id": "Triad",
      "attribute": "Dynamism"
    }
  },
  {
    "timestamp": "2024-01-15T10:30:02Z",
    "event": {
      "type": "CreateTerm",
      "system_id": "Triad",
      "index": 0,
      "character": "Will"
    }
  },
  {
    "timestamp": "2024-01-15T10:30:03Z",
    "event": {
      "type": "CreateCoordinates",
      "system_id": "Triad",
      "index": 0,
      "coordinates": {"x": 0.0, "y": 1.0, "z": null}
    }
  }
]
```

**Implementation Requirements**:
1. Add timestamp tracking to events
2. Implement event serialization/deserialization
3. Add event log storage to `System`
4. Implement replay functionality
5. Add undo/redo operations
6. Support event stream merging

## Implementation Plan

### Phase 1: Dependencies and Infrastructure
1. **Add serialization dependencies**
   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   toml = "0.8"
   chrono = { version = "0.4", features = ["serde"] }
   ```

2. **Make core types serializable**
   - `SystemId` enum
   - `Coordinates` struct
   - `Term` and `Connective` structs
   - `StateEvent` enum

3. **Create serializable system data structures**
   - Convert `by_system` structs to use `String` instead of `&'static str`
   - Add `#[derive(Serialize, Deserialize)]` to all relevant types

### Phase 2: System Data Snapshot Format
1. **Implement snapshot serialization**
   ```rust
   impl System {
       pub fn save_system_snapshot(&self, system_id: SystemId, path: &str) -> Result<(), Error>
       pub fn load_system_snapshot(&mut self, system_id: SystemId, path: &str) -> Result<(), Error>
       pub fn export_system_as_json(&self, system_id: SystemId) -> Result<String, Error>
       pub fn import_system_from_json(&mut self, system_id: SystemId, json: &str) -> Result<(), Error>
   }
   ```

2. **Create system data extractors**
   - Extract complete system state into serializable format
   - Reconstruct system from serialized data
   - Validate data integrity

3. **Add format conversion utilities**
   - Convert between JSON and TOML
   - Convert between static `by_system` files and dynamic snapshots

### Phase 3: Event Log Format
1. **Implement event logging**
   ```rust
   impl System {
       pub fn save_event_log(&self, path: &str) -> Result<(), Error>
       pub fn load_event_log(&mut self, path: &str) -> Result<(), Error>
       pub fn replay_events(&mut self, events: Vec<TimestampedEvent>) -> Result<(), Error>
       pub fn get_event_history(&self, system_id: Option<SystemId>) -> Vec<TimestampedEvent>
   }
   ```

2. **Add timestamped event structure**
   ```rust
   #[derive(Serialize, Deserialize)]
   pub struct TimestampedEvent {
       pub timestamp: DateTime<Utc>,
       pub event: StateEvent,
   }
   ```

3. **Implement undo/redo functionality**
   - Track event history in memory
   - Support event reversal
   - Maintain undo/redo stacks

### Phase 4: Library Integration
1. **Create persistence manager**
   ```rust
   pub struct PersistenceManager {
       pub system: System,
       pub event_log: Vec<TimestampedEvent>,
       pub undo_stack: Vec<TimestampedEvent>,
       pub redo_stack: Vec<TimestampedEvent>,
   }
   ```

2. **Add convenience methods**
   - Auto-save functionality
   - Backup/restore operations
   - Format conversion utilities
   - Validation and integrity checks

3. **Create file management utilities**
   - Directory structure for saved systems
   - File naming conventions
   - Version management

## API Design

### Core Persistence Interface
```rust
pub trait Persistence {
    // System Data Snapshot Format
    fn save_system_snapshot(&self, system_id: SystemId, path: &str) -> Result<(), Error>;
    fn load_system_snapshot(&mut self, system_id: SystemId, path: &str) -> Result<(), Error>;
    fn export_system_as_json(&self, system_id: SystemId) -> Result<String, Error>;
    fn import_system_from_json(&mut self, system_id: SystemId, json: &str) -> Result<(), Error>;
    
    // Event Log Format
    fn save_event_log(&self, path: &str) -> Result<(), Error>;
    fn load_event_log(&mut self, path: &str) -> Result<(), Error>;
    fn replay_events(&mut self, events: Vec<TimestampedEvent>) -> Result<(), Error>;
    fn get_event_history(&self, system_id: Option<SystemId>) -> Vec<TimestampedEvent>;
    
    // Undo/Redo
    fn undo(&mut self) -> Result<(), Error>;
    fn redo(&mut self) -> Result<(), Error>;
    fn can_undo(&self) -> bool;
    fn can_redo(&self) -> bool;
}
```

### File Structure
```
saved_systems/
├── snapshots/
│   ├── triad.json
│   ├── tetrad.json
│   └── custom_system.json
├── event_logs/
│   ├── triad_events.json
│   ├── tetrad_events.json
│   └── session_events.json
└── backups/
    ├── triad_backup_2024-01-15.json
    └── system_backup_2024-01-15.json
```

## Benefits and Use Cases

### System Data Snapshot Format
- **Fast Loading**: Direct state reconstruction without event replay
- **Human Readable**: Easy to edit manually in text editors
- **Version Control Friendly**: Clear diffs in git
- **Sharing**: Easy to share system definitions
- **Backup**: Complete system state in single file

### Event Log Format
- **Audit Trail**: See exactly what changed and when
- **Undo/Redo**: Full history of operations
- **Debugging**: Trace exact sequence of state changes
- **Collaboration**: Merge event streams from multiple users
- **Incremental**: Append new events without full state save
- **Recovery**: Reconstruct state from any point in history

## Migration Strategy

### From Static `by_system` Files
1. **Phase 1**: Keep static files as reference, add dynamic serialization
2. **Phase 2**: Convert static files to serializable format
3. **Phase 3**: Add migration utilities to convert old format
4. **Phase 4**: Deprecate static files in favor of dynamic persistence

### Backward Compatibility
- Maintain ability to load existing `by_system` files
- Provide conversion utilities
- Support both static and dynamic formats during transition

## Testing Strategy

### Unit Tests
- Serialization/deserialization of all data types
- Event replay functionality
- Undo/redo operations
- Format conversion utilities

### Integration Tests
- End-to-end save/load workflows
- Event log replay scenarios
- Multi-format compatibility
- Performance benchmarks

### Example Usage Tests
- Real-world persistence scenarios
- Large system performance
- Error handling and recovery

## Future Enhancements

### Advanced Features
- **Compression**: Compress large event logs
- **Encryption**: Secure storage of sensitive system data
- **Cloud Sync**: Remote persistence and collaboration
- **Real-time Collaboration**: Live event streaming
- **Schema Evolution**: Handle format version changes

### Performance Optimizations
- **Lazy Loading**: Load system data on demand
- **Caching**: Cache frequently accessed data
- **Batch Operations**: Optimize bulk save/load operations
- **Incremental Sync**: Only sync changed data

## Conclusion

This persistence and library feature plan provides a comprehensive approach to saving and loading system data in multiple formats. The dual-format approach ensures flexibility for different use cases while maintaining the existing architecture's strengths.

The implementation can be phased to provide immediate value while building toward a complete persistence solution. The event-driven format particularly leverages the existing architecture's strengths and provides powerful capabilities for audit trails and collaborative editing. 