//! crusti_g2io: a Graph Generator following an Inner/Outer pattern.

#![warn(missing_docs)]

mod core;
pub use crate::core::Graph;
pub use crate::core::InnerOuterGenerationStep;
pub use crate::core::InnerOuterGenerator;
pub use crate::core::InterGraphEdge;
pub use crate::core::NamedParam;
pub use crate::core::NodeIndexType;
pub use crate::core::ParameterType;
pub use crate::core::ParameterValue;

pub mod display;

pub mod generators;
pub use generators::BarabasiAlbertGeneratorFactory;
pub use generators::ChainGeneratorFactory;
pub use generators::ErdosRenyiGeneratorFactory;
pub use generators::TreeGeneratorFactory;
pub use generators::WattsStrogatzGeneratorFactory;

pub mod linkers;
pub use linkers::{BidirectionalFirstToFirstLinker, FirstToFirstLinker};
