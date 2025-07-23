use crate::core::state_manager::{Index, Coordinates};

pub struct K2Geometry;

impl K2Geometry {
    pub const INDEXES: [Index; 2] = [0, 1];
    pub const COORDINATES: [Coordinates; 2] = [
        Coordinates { x: -1.0, y: 0.0, z: None }, // Left
        Coordinates { x: 1.0, y: 0.0, z: None },  // Right
    ];
    pub const EDGES: [(Index, Index); 1] = [
        (0, 1), // Single edge
    ];
} 