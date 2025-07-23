pub mod k1;
pub mod k2;
pub mod k3;
pub mod k4;
pub mod k5;
pub mod k6;
pub mod k7;
pub mod k8;
pub mod k9;
pub mod k10;
pub mod k11;
pub mod k12;

use crate::core::state_manager::{Index, IndexPair, Coordinates};
use crate::core::traits::GeometryData;

// Re-export all geometry structs
pub use k1::K1Geometry;
pub use k2::K2Geometry;
pub use k3::K3Geometry;
pub use k4::K4Geometry;
pub use k5::K5Geometry;
pub use k6::K6Geometry;
pub use k7::K7Geometry;
pub use k8::K8Geometry;
pub use k9::K9Geometry;
pub use k10::K10Geometry;
pub use k11::K11Geometry;
pub use k12::K12Geometry;

// Implement GeometryData for all geometry structs
impl GeometryData for K1Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K2Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K3Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K4Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K5Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K6Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K7Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K8Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K9Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K10Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K11Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl GeometryData for K12Geometry {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn coordinates(&self) -> &[Coordinates] { &Self::COORDINATES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
} 