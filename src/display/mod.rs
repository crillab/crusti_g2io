//! A module dedicated to the graph output formats.
//!
//! In this module, each file is dedicated to an output format.
//! For each format, an object taking a reference to a graph is defined.
//! This object implements the [`Display`](std::fmt::Display) trait in order to be written as expected for the underlying output format.
//! Then, a new function is defined for the [`Graph`](crate::Graph) type, which returns an instance of the new object.

mod dot;

mod graphml;

mod dimacs;
