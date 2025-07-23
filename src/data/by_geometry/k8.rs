use crate::core::state_manager::{Index, Coordinates};

pub struct K8Geometry;

impl K8Geometry {
    pub const INDEXES: [Index; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    pub const COORDINATES: [Coordinates; 8] = [
        Coordinates { x: 1.0, y: 0.0, z: None },                        // 0: Smallest Significant Holon (right, middle)
        Coordinates { x: 0.70710678118, y: -0.70710678118, z: None },   // 1: Critical Functions (lower right)
        Coordinates { x: 0.0, y: -1.0, z: None },                       // 2: Supportive Platform (bottom)
        Coordinates { x: -0.70710678118, y: -0.70710678118, z: None },  // 3: Necessary Resourcing (lower left)
        Coordinates { x: -1.0, y: 0.0, z: None },                       // 4: Integrative Totality (left, middle)
        Coordinates { x: -0.70710678118, y: 0.70710678118, z: None },   // 5: Inherent Values (upper left)
        Coordinates { x: 0.0, y: 1.0, z: None },                        // 6: Intrinsic Nature (top)
        Coordinates { x: 0.70710678118, y: 0.70710678118, z: None },    // 7: Organisational Modes (upper right)
    ];
    pub const EDGES: [(Index, Index); 28] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
        (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
        (3, 4), (3, 5), (3, 6), (3, 7),
        (4, 5), (4, 6), (4, 7),
        (5, 6), (5, 7),
        (6, 7),
    ];
} 