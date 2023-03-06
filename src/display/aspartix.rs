use super::{BoxedDisplay, GraphDisplay};
use crate::{NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::Directed;

#[derive(Default)]
pub struct AspartixGraphDisplay;

impl NamedParam<BoxedDisplay<Directed>> for AspartixGraphDisplay {
    fn name(&self) -> &'static str {
        "apx"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Output a graph using the Aspartix format."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(
        &self,
        _parameter_values: Vec<ParameterValue>,
    ) -> Result<BoxedDisplay<Directed>> {
        Ok(Box::new(|f, g| {
            (0..g.n_nodes()).try_for_each(|i| writeln!(f, "arg(a{}).", i))?;
            g.iter_edges()
                .try_for_each(|e| writeln!(f, "att(a{},a{}).", e.0, e.1))
        }))
    }
}

impl GraphDisplay<Directed> for AspartixGraphDisplay {}
