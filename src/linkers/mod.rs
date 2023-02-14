//! A module dedicated to linkers.
//!
//! Linkers choose how are ade the links between inner graphs.
//!
//! ```
//! # use crusti_g2io::linkers;
//! // building a linker that links the first nodes on each graph together.
//! let linker = linkers::linker_from_str("first_bi").unwrap();
//! // the linker can then be used to link graphs
//! ```

mod first_to_first;
pub use first_to_first::{BidirectionalFirstToFirstLinker, FirstToFirstLinker};

mod min_incoming;
pub use min_incoming::{BidirectionalMinIncomingLinker, MinIncomingLinker};

use crate::{
    core::{named_param, InnerGraph},
    InterGraphEdge, NamedParam,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;

/// A boxed function that take two graphs and return a set of edges that can be used to link them.
/// The choice of the edges depends on the implementation of the linker.
///
/// ```
/// # use crusti_g2io::linkers;
/// // getting a boxed linker from a string
/// let linker = linkers::linker_from_str("first").unwrap();
/// ```
pub type BoxedLinker = Box<dyn Fn(InnerGraph, InnerGraph) -> Vec<InterGraphEdge> + Sync>;

/// A trait for objects that are used to link inner graphs.
pub trait Linker: NamedParam<BoxedLinker> {}

lazy_static! {
    pub(crate) static ref LINKERS: [Box<dyn Linker + Sync>; 4] = [
        Box::new(FirstToFirstLinker),
        Box::new(BidirectionalFirstToFirstLinker),
        Box::new(MinIncomingLinker::default()),
        Box::new(BidirectionalMinIncomingLinker::default()),
    ];
}

/// Iterates over all the linkers.
///
/// ```
/// # use crusti_g2io::linkers;
/// linkers::iter_linkers().enumerate().for_each(|(i,l)| {
///     println!(r#"linker {} has name "{}""#, i, l.name());
/// });
/// ```
pub fn iter_linkers() -> impl Iterator<Item = &'static (dyn Linker + Sync + 'static)> + 'static {
    LINKERS.iter().map(|b| b.as_ref())
}

/// Given a string representing a parameterized linker, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::linkers;
/// assert!(linkers::linker_from_str("first").is_ok()); // OK
/// assert!(linkers::linker_from_str("first/1").is_err()); // wrong parameters
/// assert!(linkers::linker_from_str("foo").is_err()); // unknown linker
/// ```
pub fn linker_from_str(s: &str) -> Result<BoxedLinker> {
    named_param::named_from_str(LINKERS.as_slice(), s)
        .context("while building a linker from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_ok() {
        assert!(linker_from_str("first").is_ok());
    }

    #[test]
    fn test_unknown_linker() {
        assert!(linker_from_str("foo/1").is_err());
    }

    #[test]
    fn test_linker_too_much_params() {
        assert!(linker_from_str("first/1").is_err());
    }
}
