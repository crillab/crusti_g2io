use crate::Graph;
use petgraph_graphml::GraphMl;
use std::fmt::Display;

impl Graph {
    /// Returns an object used to display the graph using the [GraphML format](https://en.wikipedia.org/wiki/GraphML).
    pub fn to_graphml_display(&self) -> GraphMLDisplay {
        GraphMLDisplay(self)
    }
}

pub struct GraphMLDisplay<'a>(&'a Graph);

impl Display for GraphMLDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graphml = GraphMl::new(self.0.petgraph()).pretty_print(true);
        graphml.fmt(f)
    }
}
