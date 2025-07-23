use crate::core::state_manager::{Index, Coordinates};

pub struct K3Geometry;

impl K3Geometry {
    pub const INDEXES: [Index; 3] = [0, 1, 2];
    pub const COORDINATES: [Coordinates; 3] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // Will (top left)
        Coordinates { x: 0.0, y: -1.0, z: None },  // Function (bottom left)
        Coordinates { x: 1.0, y: 0.0, z: None },   // Being (right, midpoint vertically)
    ];
    pub const EDGES: [(Index, Index); 3] = [
        (0, 1),  // Will-Function (vertical left)
        (1, 2),  // Function-Being (bottom right)
        (2, 0),  // Being-Will (top right)
    ];
} 