use super::{BoxedGenerator, GeneratorFactory};
use crate::{core::utils, NamedParam};
use anyhow::{Context, Result};
use rand::Rng;

/// A factory used to build generators for [Erdős–Rényi](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model) graphs.
///
/// Such factories can be created by passing `er/n,p` to [`generators::generator_factory_from_str`](crate::generators#generator_factory_from_str) where
///   - `n` is the size of graph to produce;
///   - `p` is the probability each edge appears in the graph.
///
/// Graphs used for initialization are star graphs.
/// Parameters must be higher than zero, and `p` must be a floating point number between 0 and 1.
#[derive(Default)]
pub struct ErdosRenyiGeneratorFactory;

impl<R> NamedParam<BoxedGenerator<R>> for ErdosRenyiGeneratorFactory
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "er"
    }

    fn description(&self) -> Vec<&'static str> {
        vec![
            "A generator following the Erdős–Rényi model.",
            "First parameter gives the number of nodes of the graph, while the second one gives the probability each edge appears in the graph."
        ]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<R>> {
        let context = "while building an Erdős–Rényi generator";
        let (n, p) =
            utils::str_param_to_positive_integer_and_probability(params).context(context)?;
        Ok(Box::new(move |r| {
            petgraph_gen::random_gnp_graph(r, n, p).into()
        }))
    }
}

impl<R> GeneratorFactory<R> for ErdosRenyiGeneratorFactory where R: Rng {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeIndexType;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_not_enough_params() {
        assert!((ErdosRenyiGeneratorFactory.try_with_params("1")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_too_much_params() {
        assert!((ErdosRenyiGeneratorFactory.try_with_params("2,1,0")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_probability_0() {
        let mut rng = rand::thread_rng();
        let g = ErdosRenyiGeneratorFactory.try_with_params("3,0").unwrap()(&mut rng);
        assert_eq!(3, g.n_nodes());
        assert_eq!(
            vec![] as Vec<(NodeIndexType, NodeIndexType)>,
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }

    #[test]
    fn test_probability_1() {
        let mut rng = rand::thread_rng();
        let g = ErdosRenyiGeneratorFactory.try_with_params("3,1").unwrap()(&mut rng);
        assert_eq!(3, g.n_nodes());
        let mut edges = g
            .iter_edges()
            .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
        edges.sort_unstable();
        assert_eq!(vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)], edges);
    }
}