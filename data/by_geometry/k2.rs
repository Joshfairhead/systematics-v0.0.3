use crate::core::state_manager::Coordinates;

pub struct K2Geometry;

impl K2Geometry {
    pub const COORDINATES: [Coordinates; 2] = [
        Coordinates { x: -1.0, y: 0.0, z: None }, // Left
        Coordinates { x: 1.0, y: 0.0, z: None },  // Right
    ];
} 