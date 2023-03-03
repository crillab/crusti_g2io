use super::{BoxedDisplay, GraphDisplay};
use crate::{NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::EdgeType;
use petgraph_graphml::GraphMl;
use std::fmt::Display;

#[derive(Default)]
pub struct GraphMLGraphDisplay;

impl<Ty> NamedParam<BoxedDisplay<Ty>> for GraphMLGraphDisplay
where
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "graphml"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Output a graph using the GraphML."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(&self, _parameter_values: Vec<ParameterValue>) -> Result<BoxedDisplay<Ty>> {
        Ok(Box::new(|f, g| {
            let graphml = GraphMl::new(g.petgraph()).pretty_print(true);
            graphml.fmt(f)
        }))
    }
}

impl<Ty> GraphDisplay<Ty> for GraphMLGraphDisplay where Ty: EdgeType {}
