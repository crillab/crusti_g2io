use crate::graph::Graph;
use petgraph::dot::{Config, Dot};
use petgraph_graphml::GraphMl;
use std::fmt::Display;

impl Graph {
    pub fn to_graphml_display(&self) -> GraphMLDisplay {
        GraphMLDisplay(self)
    }

    pub fn to_dot_display(&self) -> DotDisplay {
        DotDisplay(self)
    }
}

pub struct GraphMLDisplay<'a>(&'a Graph);

impl Display for GraphMLDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graphml = GraphMl::new(self.0.petgraph()).pretty_print(true);
        graphml.fmt(f)
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
