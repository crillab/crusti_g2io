use crate::Graph;
use petgraph::{
    dot::{Config, Dot},
    EdgeType,
};
use std::fmt::Display;

impl<Ty> Graph<Ty>
where
    Ty: EdgeType,
{
    /// Returns an object used to display the graph using the [Graphviz DOT format](https://graphviz.org/doc/info/lang.html).
    pub fn to_dot_display(&self) -> DotDisplay<Ty> {
        DotDisplay(self)
    }
}

pub struct DotDisplay<'a, Ty>(&'a Graph<Ty>)
where
    Ty: EdgeType;

impl<Ty> Display for DotDisplay<'_, Ty>
where
    Ty: EdgeType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dot_display = Dot::with_config(
            self.0.petgraph(),
            &[Config::NodeIndexLabel, Config::EdgeNoLabel],
        );
        std::fmt::Debug::fmt(&dot_display, f)
    }
}
