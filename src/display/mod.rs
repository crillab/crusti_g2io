//! A module dedicated to the formats used to display graphs.
//!
//! # Setting up a new display engine
//!
//! Setting up a new display engine is pretty similar to [setting up a new generator factory](crate::generators).
//!
//! The first difference is that all the files are located in `src/display` instead of `src/generators`.
//! The second one is that the empty trait to "implement" is [`GraphDisplay`] and not [`crate::generators::GeneratorFactory`].
//! The third one is the name of the collections of display engines in `src/display/mod.rs` (`DISPLAY_UNDIRECTED` and `DISPLAY_DIRECTED`).
//! The only real difference is in the implementation of the `try_with_params` function:
//! for display engines, the returned closure takes a [`Formatter`](std::fmt::Formatter) and a graph and display the graph, returning a formatter result.

mod dot;
use dot::DotGraphDisplay;

mod graphml;
use graphml::GraphMLGraphDisplay;

mod iccma_dimacs;
use iccma_dimacs::ICCMADimacsGraphDisplay;

use crate::{core::named_param, Graph, NamedParam};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use petgraph::{Directed, EdgeType, Undirected};

/// A boxed function that take a formatter and a graph and display the graph using the formatter.
/// The display format depends on the implementation of the display engine.
pub type BoxedDisplay<Ty> =
    Box<dyn Fn(&mut std::fmt::Formatter<'_>, &Graph<Ty>) -> std::fmt::Result>;

/// A trait for objects that are used to display graphs.
pub trait GraphDisplay<Ty>: NamedParam<BoxedDisplay<Ty>>
where
    Ty: EdgeType,
{
}

lazy_static! {
    pub(crate) static ref DISPLAY_DIRECTED: [Box<dyn GraphDisplay<Directed> + Sync>; 3] = [
        Box::new(DotGraphDisplay::default()),
        Box::new(GraphMLGraphDisplay::default()),
        Box::new(ICCMADimacsGraphDisplay::default())
    ];
}

lazy_static! {
    pub(crate) static ref DISPLAY_UNDIRECTED: [Box<dyn GraphDisplay<Undirected> + Sync>; 3] = [
        Box::new(DotGraphDisplay::default()),
        Box::new(GraphMLGraphDisplay::default()),
        Box::new(ICCMADimacsGraphDisplay::default())
    ];
}

/// Iterates over all the display engines for directed graphs.
pub fn iter_directed_display_engines(
) -> impl Iterator<Item = &'static (dyn GraphDisplay<Directed> + Sync + 'static)> + 'static {
    DISPLAY_DIRECTED.iter().map(|b| b.as_ref())
}

/// Iterates over all the display engines for undirected graphs.
pub fn iter_undirected_display_engines(
) -> impl Iterator<Item = &'static (dyn GraphDisplay<Undirected> + Sync + 'static)> + 'static {
    DISPLAY_UNDIRECTED.iter().map(|b| b.as_ref())
}

/// Given a string representing a display engine for directed graphs, returns the corresponding object.
pub fn directed_display_engine_from_str(s: &str) -> Result<BoxedDisplay<Directed>> {
    named_param::named_from_str(DISPLAY_DIRECTED.as_slice(), s)
        .context("while building a display engine from a string")
}

/// Given a string representing a display engine for undirected graphs, returns the corresponding object.
pub fn undirected_display_engine_from_str(s: &str) -> Result<BoxedDisplay<Undirected>> {
    named_param::named_from_str(DISPLAY_UNDIRECTED.as_slice(), s)
        .context("while building a display engine from a string")
}
