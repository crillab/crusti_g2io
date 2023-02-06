//! crusti_g2io: a Graph Generator following an Inner/Outer pattern.

#![warn(missing_docs)]

mod display;

pub mod generators;
pub use generators::BarabasiAlbertGeneratorFactory;
pub use generators::ChainGeneratorFactory;

mod graph;
pub use graph::Graph;
pub use graph::InterGraphEdge;
pub use graph::NodeIndexType;

pub mod linkers;
pub use linkers::{BidirectionalFirstToFirstLinker, FirstToFirstLinker};

mod utils;
pub use utils::NamedParam;
