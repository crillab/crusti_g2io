use super::{BoxedGenerator, GeneratorFactory};
use crate::{NamedParam, ParameterType, ParameterValue};
use anyhow::{anyhow, Context, Result};
use petgraph::EdgeType;
use rand::Rng;

/// A factory used to build generators for [Barabási-Albert](https://en.wikipedia.org/wiki/Barab%C3%A1si%E2%80%93Albert_model) graphs.
///
/// In directed graphs generated with this object, edge sources are the new nodes and targets the existing ones.
///
/// Such factories can be created by passing `ba/n,m` to [`generators::generator_factory_from_str`](crate::generators#generator_factory_from_str) where
///   - `n` is the size of graph to produce;
///   - `m` is the size of the graph used for the initialization step.
///
/// Graphs used for initialization are star graphs.
/// Both parameters must be higher than zero, and `n` must be higher than `m`.
#[derive(Default)]
pub struct BarabasiAlbertGeneratorFactory;

impl<Ty, R> NamedParam<BoxedGenerator<Ty, R>> for BarabasiAlbertGeneratorFactory
where
    R: Rng,
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "ba"
    }

    fn description(&self) -> Vec<&'static str> {
        vec![
            "A generator following the Barabási-Albert model, initialized by a star graph.",
            "First parameter gives the number of nodes of the graph, while the second one gives the number of nodes of the initial star graph."
        ]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![
            ParameterType::PositiveInteger,
            ParameterType::PositiveInteger,
        ]
    }

    fn try_with_params(
        &self,
        parameter_values: Vec<ParameterValue>,
    ) -> Result<BoxedGenerator<Ty, R>> {
        let context = "while building a Barabasi-Albert generator";
        let n = parameter_values[0].unwrap_usize();
        let m = parameter_values[1].unwrap_usize();
        if m == 0 || m >= n {
            return Err(anyhow!(
                r#"second parameter ("m") must be higher than 0 and lower than the first one ("n")"#
            ))
            .context(context);
        }
        Ok(Box::new(move |r| {
            petgraph_gen::barabasi_albert_graph(r, n, m, None).into()
        }))
    }
}

impl<Ty, R> GeneratorFactory<Ty, R> for BarabasiAlbertGeneratorFactory
where
    R: Rng,
    Ty: EdgeType,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Graph, NodeIndexType};
    use petgraph::Directed;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_m_is_zero() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params(vec![
            ParameterValue::PositiveInteger(2),
            ParameterValue::PositiveInteger(0)
        ]) as Result<BoxedGenerator<Directed, ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_n_is_not_higher_than_m() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params(vec![
            ParameterValue::PositiveInteger(2),
            ParameterValue::PositiveInteger(2)
        ]) as Result<BoxedGenerator<Directed, ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_barabasi_star() {
        let mut rng = rand::thread_rng();
        let g: Graph<Directed> = BarabasiAlbertGeneratorFactory
            .try_with_params(vec![
                ParameterValue::PositiveInteger(4),
                ParameterValue::PositiveInteger(3),
            ])
            .unwrap()(&mut rng);
        assert_eq!(
            vec![(0, 1), (0, 2), (0, 3)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }
}
