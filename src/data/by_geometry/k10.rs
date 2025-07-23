use crate::core::state_manager::{Index, Coordinates};

pub struct K10Geometry;

impl K10Geometry {
    pub const INDEXES: [Index; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    pub const COORDINATES: [Coordinates; 10] = [
        Coordinates { x: 0.0, y: 1.0, z: None },
        Coordinates { x: 0.58778525229, y: 0.80901699437, z: None },
        Coordinates { x: 0.95105651630, y: 0.30901699437, z: None },
        Coordinates { x: 0.95105651630, y: -0.30901699437, z: None },
        Coordinates { x: 0.58778525229, y: -0.80901699437, z: None },
        Coordinates { x: 0.0, y: -1.0, z: None },
        Coordinates { x: -0.58778525229, y: -0.80901699437, z: None },
        Coordinates { x: -0.95105651630, y: -0.30901699437, z: None },
        Coordinates { x: -0.95105651630, y: 0.30901699437, z: None },
        Coordinates { x: -0.58778525229, y: 0.80901699437, z: None },
    ];
    pub const EDGES: [(Index, Index); 45] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9),
        (2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9),
        (3, 4), (3, 5), (3, 6), (3, 7), (3, 8), (3, 9),
        (4, 5), (4, 6), (4, 7), (4, 8), (4, 9),
        (5, 6), (5, 7), (5, 8), (5, 9),
        (6, 7), (6, 8), (6, 9),
        (7, 8), (7, 9),
        (8, 9),
    ];
} 