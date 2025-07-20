//! Core types and interfaces for all compositional systems.
//!
//! This module provides the central state manager, default system data trait,
//! and generic CRUD interfaces that work with any system ID, allowing
//! consistent operations across all compositional systems.

pub mod default_system_data;
pub mod state_manager;
pub mod term_characters;
pub mod connective_characters;
pub mod designations;
pub mod system_manager;

// Re-export main types and traits for convenience
pub use default_system_data::DefaultSystemData;
pub use state_manager::{System, SystemId, Coordinates, StateEvent};
pub use term_characters::{Term, TermCharacters};
pub use connective_characters::{Connective, ConnectiveCharacters};
pub use designations::Designations;
pub use system_manager::SystemManager; 