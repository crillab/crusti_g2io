use super::{BoxedLinker, Linker};
use crate::{InterGraphEdge, NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::{Directed, EdgeType};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

/// A linker that connects the nodes from the first graph to the ones of the second graph in a random fashion.
///
/// In order to randomly connect the graphs, all edges are considered and added with the probability given by the dedicated parameter.
///
/// Such linker can be created by passing `random` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct RandomLinker;

impl<Ty, R> NamedParam<BoxedLinker<Ty, R>> for RandomLinker
where
    R: Rng,
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "random"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes from the first graph to the ones of the second graph in a random fashion.", "The probability each arc is set is given by the first parameter."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::Probability]
    }

    fn try_with_params(&self, parameter_values: Vec<ParameterValue>) -> Result<BoxedLinker<Ty, R>> {
        try_with_params(parameter_values, false)
    }
}

impl<Ty, R> Linker<Ty, R> for RandomLinker
where
    R: Rng,
    Ty: EdgeType,
{
}

/// A bidirectional linker that connects the nodes from one graph to another in a random fashion.
///
/// In order to randomly connect the graphs, all edges are considered and added with the probability given by the dedicated parameter.
///
/// Such linker can be created by passing `random_bi` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct BidirectionalRandomLinker;

impl<R> NamedParam<BoxedLinker<Directed, R>> for BidirectionalRandomLinker
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "random_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes from the first graph to the ones of the second graph in a random fashion, and vice-versa.", "The probability each arc is set is given by the first parameter."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![ParameterType::Probability]
    }

    fn try_with_params(
        &self,
        parameter_values: Vec<ParameterValue>,
    ) -> Result<BoxedLinker<Directed, R>> {
        try_with_params(parameter_values, true)
    }
}

impl<R> Linker<Directed, R> for BidirectionalRandomLinker where R: Rng {}

fn try_with_params<Ty, R>(
    parameter_values: Vec<ParameterValue>,
    bidirectional: bool,
) -> Result<BoxedLinker<Ty, R>>
where
    R: Rng,
    Ty: EdgeType,
{
    let p = parameter_values[0].unwrap_f64();
    Ok(Box::new(move |g1, g2, rng| {
        let proba_uniform = Uniform::new_inclusive(0., 1.);
        let mut estimated_cap = p * g1.graph().n_nodes() as f64 * g2.graph().n_nodes() as f64;
        if bidirectional {
            estimated_cap *= 2.;
        }
        let mut edges = Vec::with_capacity(estimated_cap as usize);
        for i in 0..g1.graph().n_nodes() {
            for j in 0..g2.graph().n_nodes() {
                if proba_uniform.sample(rng) < p {
                    edges.push(InterGraphEdge::FirstToSecond(i, j));
                }
                if bidirectional && proba_uniform.sample(rng) < p {
                    edges.push(InterGraphEdge::SecondToFirst(j, i));
                }
            }
        }
        edges.shrink_to_fit();
        edges
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::{BoxedGenerator, ChainGeneratorFactory};
    use petgraph::Directed;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_random_ok_0() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = ChainGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = RandomLinker
            .try_with_params(vec![ParameterValue::Probability(0.0)])
            .unwrap();
        assert_eq!(
            vec![] as Vec<InterGraphEdge>,
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_random_ok_1() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = ChainGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = RandomLinker
            .try_with_params(vec![ParameterValue::Probability(1.0)])
            .unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::FirstToSecond(0, 1),
                InterGraphEdge::FirstToSecond(1, 0),
                InterGraphEdge::FirstToSecond(1, 1),
            ],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_random_bi_ok_0() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = ChainGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalRandomLinker
            .try_with_params(vec![ParameterValue::Probability(0.0)])
            .unwrap();
        assert_eq!(
            vec![] as Vec<InterGraphEdge>,
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_random_bi_ok_1() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = ChainGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalRandomLinker
            .try_with_params(vec![ParameterValue::Probability(1.0)])
            .unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0),
                InterGraphEdge::FirstToSecond(0, 1),
                InterGraphEdge::SecondToFirst(1, 0),
                InterGraphEdge::FirstToSecond(1, 0),
                InterGraphEdge::SecondToFirst(0, 1),
                InterGraphEdge::FirstToSecond(1, 1),
                InterGraphEdge::SecondToFirst(1, 1),
            ],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }
}
