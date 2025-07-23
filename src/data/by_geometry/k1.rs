use crate::core::state_manager::{Index, Coordinates};

pub struct K1Geometry;

impl K1Geometry {
    pub const INDEXES: [Index; 1] = [0];
    pub const COORDINATES: [Coordinates; 1] = [
        Coordinates { x: 0.0, y: 0.0, z: None }, // Single point
    ];
    pub const EDGES: [(Index, Index); 0] = [];
} 