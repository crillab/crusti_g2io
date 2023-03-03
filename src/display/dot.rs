use super::{BoxedDisplay, GraphDisplay};
use crate::{NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::{
    dot::{Config, Dot},
    EdgeType,
};

#[derive(Default)]
pub struct DotGraphDisplay;

impl<Ty> NamedParam<BoxedDisplay<Ty>> for DotGraphDisplay
where
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "dot"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Output a graph using the Graphviz DOT format."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(&self, _parameter_values: Vec<ParameterValue>) -> Result<BoxedDisplay<Ty>> {
        Ok(Box::new(|f, g| {
            let dot_display =
                Dot::with_config(g.petgraph(), &[Config::NodeIndexLabel, Config::EdgeNoLabel]);
            std::fmt::Debug::fmt(&dot_display, f)
        }))
    }
}

impl<Ty> GraphDisplay<Ty> for DotGraphDisplay where Ty: EdgeType {}
