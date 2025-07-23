use crate::core::state_manager::{Index, Coordinates};

pub struct K5Geometry;

impl K5Geometry {
    pub const INDEXES: [Index; 5] = [0, 1, 2, 3, 4];
    pub const COORDINATES: [Coordinates; 5] = [
        Coordinates { x: 1.0, y: 1.0, z: None },     // 0: Purpose (upper right)
        Coordinates { x: 0.5, y: 0.5, z: None },     // 1: Higher Potential (middle, above quintessence, below purpose)
        Coordinates { x: -1.0, y: 0.0, z: None },    // 2: Quintessence (left, vertical midpoint)
        Coordinates { x: 0.5, y: -0.5, z: None },    // 3: Lower Potential (middle, below quintessence, above source)
        Coordinates { x: 1.0, y: -1.0, z: None },    // 4: Source (lower right)
    ];
    pub const EDGES: [(Index, Index); 10] = [
        (0, 1), // Purpose-Higher Potential
        (1, 2), // Higher Potential-Quintessence
        (2, 3), // Quintessence-Lower Potential
        (3, 4), // Lower Potential-Source
        (4, 0), // Source-Purpose
        (0, 2), // Purpose-Quintessence
        (1, 3), // Higher Potential-Lower Potential
        (2, 4), // Quintessence-Source
        (3, 0), // Lower Potential-Purpose
        (4, 1), // Source-Higher Potential
    ];
} 