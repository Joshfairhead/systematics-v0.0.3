use crate::core::state_manager::Index;

pub struct K3Topology;

impl K3Topology {
    pub const INDEXES: [Index; 3] = [0, 1, 2];
    pub const EDGES: [(Index, Index); 3] = [
        (0, 1),  // Will-Function (vertical left)
        (1, 2),  // Function-Being (bottom right)
        (2, 0),  // Being-Will (top right)
    ];
}
