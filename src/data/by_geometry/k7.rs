use crate::core::state_manager::{Index, Coordinates};

pub struct K7Geometry;

impl K7Geometry {
    pub const INDEXES: [Index; 7] = [0, 1, 2, 3, 4, 5, 6];
    pub const COORDINATES: [Coordinates; 7] = [
        Coordinates { x: 0.0, y: 1.0, z: None },             // 0: Insight (top center)
        Coordinates { x: 0.781831, y: 0.623489, z: None },   // 1: Research
        Coordinates { x: 0.974370, y: -0.222521, z: None },  // 2: Design
        Coordinates { x: 0.433884, y: -0.900969, z: None },  // 3: Synthesis
        Coordinates { x: -0.433884, y: -0.900969, z: None }, // 4: Application
        Coordinates { x: -0.974370, y: -0.222521, z: None }, // 5: Delivery
        Coordinates { x: -0.781831, y: 0.623489, z: None },  // 6: Value
    ];
    pub const EDGES: [(Index, Index); 21] = [
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
        (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
        (2, 3), (2, 4), (2, 5), (2, 6),
        (3, 4), (3, 5), (3, 6),
        (4, 5), (4, 6),
        (5, 6),
    ];
} 