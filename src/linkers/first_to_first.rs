use super::{BoxedLinker, Linker};
use crate::{InterGraphEdge, NamedParam, ParameterType, ParameterValue};
use anyhow::Result;
use petgraph::{Directed, EdgeType};
use rand::Rng;

/// A linker that connects first nodes.
///
/// Such linker can be created by passing `first` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct FirstToFirstLinker;

impl<Ty, R> NamedParam<BoxedLinker<Ty, R>> for FirstToFirstLinker
where
    R: Rng,
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "first"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the lowest index node of the first graph to the lowest index node of the second graph."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(
        &self,
        _parameter_values: Vec<ParameterValue>,
    ) -> Result<BoxedLinker<Ty, R>> {
        try_with_params(false)
    }
}

impl<Ty, R> Linker<Ty, R> for FirstToFirstLinker
where
    R: Rng,
    Ty: EdgeType,
{
}

/// A bidirectional linker that connects first nodes.
///
/// Such linker can be created by passing `first_bi` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct BidirectionalFirstToFirstLinker;

impl<R> NamedParam<BoxedLinker<Directed, R>> for BidirectionalFirstToFirstLinker
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "first_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the lowest index node of the first graph to the lowest index node of the second graph, and vice-versa."]
    }

    fn expected_parameter_types(&self) -> Vec<ParameterType> {
        vec![]
    }

    fn try_with_params(
        &self,
        _parameter_values: Vec<ParameterValue>,
    ) -> Result<BoxedLinker<Directed, R>> {
        try_with_params(true)
    }
}

impl<R> Linker<Directed, R> for BidirectionalFirstToFirstLinker where R: Rng {}

fn try_with_params<Ty, R>(bidirectional: bool) -> Result<BoxedLinker<Ty, R>>
where
    R: Rng,
    Ty: EdgeType,
{
    Ok(Box::new(move |_, _, _| {
        if bidirectional {
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0),
            ]
        } else {
            vec![InterGraphEdge::FirstToSecond(0, 0)]
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::{BoxedGenerator, PathGeneratorFactory};
    use petgraph::Directed;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_f2f_ok() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = PathGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = FirstToFirstLinker.try_with_params(vec![]).unwrap();
        assert_eq!(
            vec![InterGraphEdge::FirstToSecond(0, 0)],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_f2f_bi_ok() {
        let graph_generator: BoxedGenerator<Directed, ThreadRng> = PathGeneratorFactory
            .try_with_params(vec![ParameterValue::PositiveInteger(2)])
            .unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalFirstToFirstLinker
            .try_with_params(vec![])
            .unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0)
            ],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }
}
