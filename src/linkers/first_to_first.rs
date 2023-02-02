use super::{BoxedLinker, Linker};
use crate::{graph::InterGraphEdge, utils::Named};
use anyhow::{anyhow, Context, Result};

pub struct FirstToFirstLinker;

impl Named<BoxedLinker> for FirstToFirstLinker {
    fn name(&self) -> &'static str {
        "f2f"
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, false)
    }
}

impl Linker for FirstToFirstLinker {}

pub struct BidirectionalFirstToFirstLinker;

impl Named<BoxedLinker> for BidirectionalFirstToFirstLinker {
    fn name(&self) -> &'static str {
        "f2f_bi"
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, true)
    }
}

impl Linker for BidirectionalFirstToFirstLinker {}

fn try_with_params(params: &str, bidirectional: bool) -> Result<BoxedLinker> {
    let context = "while building a first-to-first linker";
    if !params.is_empty() {
        return Err(anyhow!("unexpected parameter(s)")).context(context);
    }
    Ok(Box::new(move |_, _| {
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

    #[test]
    fn test_f2f_too_much_params() {
        assert!(FirstToFirstLinker.try_with_params("1").is_err());
    }

    #[test]
    fn test_f2f_ok() {
        let graph_generator = ChainGeneratorFactory.try_with_params("2").unwrap();
        let mut rng = rand::thread_rng();
        let g0 = graph_generator(&mut rng);
        let g1 = graph_generator(&mut rng);
        let linker = FirstToFirstLinker.try_with_params("").unwrap();
        assert_eq!(vec![InterGraphEdge::FirstToSecond(0, 0)], linker(&g0, &g1));
    }

    #[test]
    fn test_f2f_bi_too_much_params() {
        assert!(BidirectionalFirstToFirstLinker
            .try_with_params("1")
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
            linker(&g0, &g1)
        );
    }
}
