use crate::Graph;
use petgraph::EdgeType;
use petgraph_graphml::GraphMl;
use std::fmt::Display;

impl<Ty> Graph<Ty>
where
    Ty: EdgeType,
{
    /// Returns an object used to display the graph using the [GraphML format](https://en.wikipedia.org/wiki/GraphML).
    pub fn to_graphml_display(&self) -> GraphMLDisplay<Ty> {
        GraphMLDisplay(self)
    }
}

pub struct GraphMLDisplay<'a, Ty>(&'a Graph<Ty>)
where
    Ty: EdgeType;

impl<Ty> Display for GraphMLDisplay<'_, Ty>
where
    Ty: EdgeType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let graphml = GraphMl::new(self.0.petgraph()).pretty_print(true);
        graphml.fmt(f)
    }
}
