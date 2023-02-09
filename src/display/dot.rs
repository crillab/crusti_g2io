use crate::Graph;
use petgraph::dot::{Config, Dot};
use std::fmt::Display;

impl Graph {
    /// Returns an object used to display the graph using the [Graphviz DOT format](https://graphviz.org/doc/info/lang.html).
    pub fn to_dot_display(&self) -> DotDisplay {
        DotDisplay(self)
    }
}

pub struct DotDisplay<'a>(&'a Graph);

impl Display for DotDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dot_display = Dot::with_config(
            self.0.petgraph(),
            &[Config::NodeIndexLabel, Config::EdgeNoLabel],
        );
        std::fmt::Debug::fmt(&dot_display, f)
    }
}
