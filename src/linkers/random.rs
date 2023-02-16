use super::{BoxedLinker, Linker};
use crate::{core::utils, InterGraphEdge, NamedParam};
use anyhow::{Context, Result};
use petgraph::EdgeType;
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

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker<Ty, R>> {
        try_with_params(params, false)
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

impl<Ty, R> NamedParam<BoxedLinker<Ty, R>> for BidirectionalRandomLinker
where
    R: Rng,
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "random_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes from the first graph to the ones of the second graph in a random fashion, and vice-versa.", "The probability each arc is set is given by the first parameter."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker<Ty, R>> {
        try_with_params(params, true)
    }
}

impl<Ty, R> Linker<Ty, R> for BidirectionalRandomLinker
where
    R: Rng,
    Ty: EdgeType,
{
}

fn try_with_params<Ty, R>(params: &str, bidirectional: bool) -> Result<BoxedLinker<Ty, R>>
where
    R: Rng,
    Ty: EdgeType,
{
    let context = "while building a random linker";
    let (_, p) =
        utils::str_param_to_positive_integers_and_probability(params, 0).context(context)?;
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
    fn test_random_not_enough_params() {
        assert!(
            (RandomLinker.try_with_params("") as Result<BoxedLinker<Directed, ThreadRng>>).is_err()
        );
    }

    #[test]
    fn test_random_too_much_params() {
        assert!(
            (RandomLinker.try_with_params("1,1") as Result<BoxedLinker<Directed, ThreadRng>>)
                .is_err()
        );
    }

    #[test]
    fn test_random_ok_0() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> =
            ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = RandomLinker.try_with_params("0").unwrap();
        assert_eq!(
            vec![] as Vec<InterGraphEdge>,
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_random_ok_1() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> =
            ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = RandomLinker.try_with_params("1").unwrap();
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
    fn test_random_bi_not_enough_params() {
        assert!((BidirectionalRandomLinker.try_with_params("")
            as Result<BoxedLinker<Directed, ThreadRng>>)
            .is_err());
    }

    #[test]
    fn test_random_bi_too_much_params() {
        assert!((BidirectionalRandomLinker.try_with_params("1,1")
            as Result<BoxedLinker<Directed, ThreadRng>>)
            .is_err());
    }

    #[test]
    fn test_random_bi_ok_0() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> =
            ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalRandomLinker.try_with_params("0").unwrap();
        assert_eq!(
            vec![] as Vec<InterGraphEdge>,
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_random_bi_ok_1() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> =
            ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalRandomLinker.try_with_params("1").unwrap();
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
