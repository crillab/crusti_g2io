//! crusti_g2io: a Graph Generator following an Inner/Outer pattern.

#![warn(missing_docs)]

mod core;
pub use crate::core::Graph;
pub use crate::core::InterGraphEdge;
pub use crate::core::NamedParam;
pub use crate::core::NodeIndexType;

mod display;

pub mod generators;
pub use generators::BarabasiAlbertGeneratorFactory;
pub use generators::ChainGeneratorFactory;
pub use generators::TreeGeneratorFactory;

pub mod linkers;
pub use linkers::{BidirectionalFirstToFirstLinker, FirstToFirstLinker};
