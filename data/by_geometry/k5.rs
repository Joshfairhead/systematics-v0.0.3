use crate::core::state_manager::Coordinates;

pub struct K5Geometry;

impl K5Geometry {
    pub const COORDINATES: [Coordinates; 5] = [
        Coordinates { x: 1.0, y: 1.0, z: None },     // 0: Purpose (upper right)
        Coordinates { x: 0.5, y: 0.5, z: None },     // 1: Higher Potential (middle, above quintessence, below purpose)
        Coordinates { x: -1.0, y: 0.0, z: None },    // 2: Quintessence (left, vertical midpoint)
        Coordinates { x: 0.5, y: -0.5, z: None },    // 3: Lower Potential (middle, below quintessence, above source)
        Coordinates { x: 1.0, y: -1.0, z: None },    // 4: Source (lower right)
    ];
} 