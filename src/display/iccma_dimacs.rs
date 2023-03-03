use super::{BoxedDisplay, GraphDisplay};
use crate::{NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::EdgeType;

#[derive(Default)]
pub struct ICCMADimacsGraphDisplay;

impl<Ty> NamedParam<BoxedDisplay<Ty>> for ICCMADimacsGraphDisplay
where
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "iccma_dimacs"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Output a graph using the DIMACS-like format used at ICCMA'23."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(&self, _parameter_values: Vec<ParameterValue>) -> Result<BoxedDisplay<Ty>> {
        Ok(Box::new(|f, g| {
            writeln!(f, "p af {}", g.n_nodes())?;
            for e in g.iter_edges() {
                writeln!(f, "{} {}", e.0 + 1, e.1 + 1)?;
                if !Ty::is_directed() {
                    writeln!(f, "{} {}", e.1 + 1, e.0 + 1)?;
                }
            }
            Ok(())
        }))
    }
}

impl<Ty> GraphDisplay<Ty> for ICCMADimacsGraphDisplay where Ty: EdgeType {}
