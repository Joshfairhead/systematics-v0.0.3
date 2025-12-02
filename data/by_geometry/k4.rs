use crate::core::state_manager::Coordinates;

pub struct K4Geometry;

impl K4Geometry {
    pub const COORDINATES: [Coordinates; 4] = [
        Coordinates { x: 0.0, y: 1.0, z: None },   // ideal
        Coordinates { x: 1.0, y: 0.0, z: None },   // directive
        Coordinates { x: -1.0, y: 0.0, z: None },  // instrumental
        Coordinates { x: 0.0, y: -1.0, z: None },  // ground
    ];
} 