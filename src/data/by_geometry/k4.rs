use crate::core::state_manager::{Index, Coordinates};

pub struct K4Geometry;

impl K4Geometry {
    pub const INDEXES: [Index; 4] = [0, 1, 2, 3];
    pub const COORDINATES: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // ideal
        Coordinates { x: 1.0, y: 0.0, z: None },   // directive
        Coordinates { x: -1.0, y: 0.0, z: None },  // instrumental
        Coordinates { x: 0.0, y: -1.0, z: None },  // ground
    ];
    pub const EDGES: [(Index, Index); 6] = [
        (0, 1),  // ideal-directive
        (0, 2),  // ideal-instrumental
        (0, 3),  // ideal-ground
        (1, 2),  // directive-instrumental
        (1, 3),  // directive-ground
        (2, 3),  // instrumental-ground
    ];
} 