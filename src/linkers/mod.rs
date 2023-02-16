//! A module dedicated to linkers.
//!
//! Linkers choose how are ade the links between inner graphs.
//!
//! ```
//! # use crusti_g2io::linkers;
//! // building a linker that links the first nodes on each graph together.
//! let linker = linkers::directed_linker_from_str("first_bi").unwrap();
//! // the linker can then be used to link graphs
//! ```

mod first_to_first;
pub use first_to_first::{BidirectionalFirstToFirstLinker, FirstToFirstLinker};

mod min_incoming;
pub use min_incoming::{BidirectionalMinIncomingLinker, MinIncomingLinker};

mod random;
pub use random::{BidirectionalRandomLinker, RandomLinker};

use crate::{
    core::{named_param, InnerGraph},
    InterGraphEdge, NamedParam,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use petgraph::{Directed, EdgeType, Undirected};
use rand_pcg::Pcg32;

/// A boxed function that take two graphs and return a set of edges that can be used to link them.
/// The choice of the edges depends on the implementation of the linker.
///
/// ```
/// # use crusti_g2io::linkers;
/// // getting a boxed linker from a string
/// let linker = linkers::directed_linker_from_str("first").unwrap();
/// ```
pub type BoxedLinker<Ty, R> =
    Box<dyn Fn(InnerGraph<Ty>, InnerGraph<Ty>, &mut R) -> Vec<InterGraphEdge> + Sync>;

/// A trait for objects that are used to link inner graphs.
pub trait Linker<Ty, R>: NamedParam<BoxedLinker<Ty, R>>
where
    Ty: EdgeType,
{
}

lazy_static! {
    pub(crate) static ref LINKERS_DIRECTED_PCG32: [Box<dyn Linker<Directed, Pcg32> + Sync>; 6] = [
        Box::new(FirstToFirstLinker::default()),
        Box::new(BidirectionalFirstToFirstLinker::default()),
        Box::new(MinIncomingLinker::default()),
        Box::new(BidirectionalMinIncomingLinker::default()),
        Box::new(RandomLinker::default()),
        Box::new(BidirectionalRandomLinker::default()),
    ];
}

lazy_static! {
    pub(crate) static ref LINKERS_UNDIRECTED_PCG32: [Box<dyn Linker<Undirected, Pcg32> + Sync>; 6] = [
        Box::new(FirstToFirstLinker::default()),
        Box::new(BidirectionalFirstToFirstLinker::default()),
        Box::new(MinIncomingLinker::default()),
        Box::new(BidirectionalMinIncomingLinker::default()),
        Box::new(RandomLinker::default()),
        Box::new(BidirectionalRandomLinker::default()),
    ];
}

/// Iterates over all the linkers for directed graphs.
///
/// ```
/// # use crusti_g2io::linkers;
/// linkers::iter_directed_linkers().enumerate().for_each(|(i,l)| {
///     println!(r#"linker {} has name "{}""#, i, l.name());
/// });
/// ```
pub fn iter_directed_linkers(
) -> impl Iterator<Item = &'static (dyn Linker<Directed, Pcg32> + Sync + 'static)> + 'static {
    LINKERS_DIRECTED_PCG32.iter().map(|b| b.as_ref())
}

/// Iterates over all the linkers for undirected graphs.
///
/// ```
/// # use crusti_g2io::linkers;
/// linkers::iter_undirected_linkers().enumerate().for_each(|(i,l)| {
///     println!(r#"linker {} has name "{}""#, i, l.name());
/// });
/// ```
pub fn iter_undirected_linkers(
) -> impl Iterator<Item = &'static (dyn Linker<Undirected, Pcg32> + Sync + 'static)> + 'static {
    LINKERS_UNDIRECTED_PCG32.iter().map(|b| b.as_ref())
}

/// Given a string representing a parameterized linker for directed graphs, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::linkers;
/// assert!(linkers::directed_linker_from_str("first").is_ok()); // OK
/// assert!(linkers::directed_linker_from_str("first/1").is_err()); // wrong parameters
/// assert!(linkers::directed_linker_from_str("foo").is_err()); // unknown linker
/// ```
pub fn directed_linker_from_str(s: &str) -> Result<BoxedLinker<Directed, Pcg32>> {
    named_param::named_from_str(LINKERS_DIRECTED_PCG32.as_slice(), s)
        .context("while building a linker from a string")
}

/// Given a string representing a parameterized linker for undirected graphs, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::linkers;
/// assert!(linkers::undirected_linker_from_str("first").is_ok()); // OK
/// assert!(linkers::undirected_linker_from_str("first/1").is_err()); // wrong parameters
/// assert!(linkers::undirected_linker_from_str("foo").is_err()); // unknown linker
/// ```
pub fn undirected_linker_from_str(s: &str) -> Result<BoxedLinker<Undirected, Pcg32>> {
    named_param::named_from_str(LINKERS_UNDIRECTED_PCG32.as_slice(), s)
        .context("while building a linker from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_ok() {
        assert!(directed_linker_from_str("first").is_ok());
    }

    #[test]
    fn test_unknown_linker() {
        assert!(directed_linker_from_str("foo/1").is_err());
    }

    #[test]
    fn test_linker_too_much_params() {
        assert!(directed_linker_from_str("first/1").is_err());
    }
}
