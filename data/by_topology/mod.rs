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

use crate::core::state_manager::{Index, IndexPair};
use crate::core::traits::TopologyData;

// Re-export all topology structs
pub use k1::K1Topology;
pub use k2::K2Topology;
pub use k3::K3Topology;
pub use k4::K4Topology;
pub use k5::K5Topology;
pub use k6::K6Topology;
pub use k7::K7Topology;
pub use k8::K8Topology;
pub use k9::K9Topology;
pub use k10::K10Topology;
pub use k11::K11Topology;
pub use k12::K12Topology;

// Implement TopologyData for all topology structs
impl TopologyData for K1Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K2Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K3Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K4Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K5Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K6Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K7Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K8Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K9Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K10Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K11Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}

impl TopologyData for K12Topology {
    fn indexes(&self) -> &[Index] { &Self::INDEXES }
    fn edges(&self) -> &[IndexPair] { &Self::EDGES }
}
