use crate::core::state_manager::Index;

pub struct K1Topology;

impl K1Topology {
    pub const INDEXES: [Index; 1] = [0];
    pub const EDGES: [(Index, Index); 0] = [];
}
