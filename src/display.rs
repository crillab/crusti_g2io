use crate::graph::Graph;
use petgraph_graphml::GraphMl;
use std::fmt::Display;

pub struct GraphMLDisplay<'a>(&'a Graph);

impl Graph {
    pub fn to_graphml_display(&self) -> GraphMLDisplay {
        GraphMLDisplay(self)
    }
}

impl Display for GraphMLDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graphml = GraphMl::new(self.0.petgraph()).pretty_print(true);
        graphml.fmt(f)
    }
}
