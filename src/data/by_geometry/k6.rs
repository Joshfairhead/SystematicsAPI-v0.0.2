use crate::core::state_manager::{Index, Coordinates};

pub struct K6Geometry;

impl K6Geometry {
    pub const INDEXES: [Index; 6] = [0, 1, 2, 3, 4, 5];
    pub const COORDINATES: [Coordinates; 6] = [
        Coordinates { x: -0.5, y: 0.86602540378, z: None }, // index 0: top left
        Coordinates { x: 0.0, y: 1.0, z: None },            // index 1: top tip
        Coordinates { x: 0.5, y: 0.86602540378, z: None },  // index 2: top right
        Coordinates { x: 1.0, y: 0.0, z: None },            // index 3: bottom right
        Coordinates { x: 0.0, y: -1.0, z: None },           // index 4: bottom
        Coordinates { x: -1.0, y: 0.0, z: None },           // index 5: bottom left
    ];
    pub const EDGES: [(Index, Index); 15] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
        (1, 2), (1, 3), (1, 4), (1, 5),
        (2, 3), (2, 4), (2, 5),
        (3, 4), (3, 5),
        (4, 5),
    ];
} 