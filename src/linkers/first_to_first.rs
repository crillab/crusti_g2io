use super::{BoxedLinker, Linker};
use crate::{InterGraphEdge, NamedParam};
use anyhow::{anyhow, Context, Result};
use rand::Rng;

/// A linker that connects first nodes.
///
/// Such linker can be created by passing `first` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct FirstToFirstLinker;

impl<R> NamedParam<BoxedLinker<R>> for FirstToFirstLinker
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "first"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the lowest index node of the first graph to the lowest index node of the second graph."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker<R>> {
        try_with_params(params, false)
    }
}

impl<R> Linker<R> for FirstToFirstLinker where R: Rng {}

/// A bidirectional linker that connects first nodes.
///
/// Such linker can be created by passing `first_bi` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct BidirectionalFirstToFirstLinker;

impl<R> NamedParam<BoxedLinker<R>> for BidirectionalFirstToFirstLinker
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "first_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the lowest index node of the first graph to the lowest index node of the second graph, and vice-versa."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker<R>> {
        try_with_params(params, true)
    }
}

impl<R> Linker<R> for BidirectionalFirstToFirstLinker where R: Rng {}

fn try_with_params<R>(params: &str, bidirectional: bool) -> Result<BoxedLinker<R>>
where
    R: Rng,
{
    let context = "while building a first-to-first linker";
    if !params.is_empty() {
        return Err(anyhow!("unexpected parameter(s)")).context(context);
    }
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
    use crate::generators::ChainGeneratorFactory;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_f2f_too_much_params() {
        assert!(
            (FirstToFirstLinker.try_with_params("1") as Result<BoxedLinker<ThreadRng>>).is_err()
        );
    }

    #[test]
    fn test_f2f_ok() {
        let graph_generator = ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = FirstToFirstLinker.try_with_params("").unwrap();
        assert_eq!(
            vec![InterGraphEdge::FirstToSecond(0, 0)],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }

    #[test]
    fn test_f2f_bi_too_much_params() {
        assert!((BidirectionalFirstToFirstLinker.try_with_params("1")
            as Result<BoxedLinker<ThreadRng>>)
            .is_err());
    }

    #[test]
    fn test_f2f_bi_ok() {
        let graph_generator = ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = BidirectionalFirstToFirstLinker.try_with_params("").unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0)
            ],
            linker((0, &g0).into(), (1, &g1).into(), &mut rand::thread_rng())
        );
    }
}
