use super::{BoxedGenerator, GeneratorFactory};
use crate::{
    graph::Graph,
    utils::{self, NamedParam},
};
use anyhow::{anyhow, Context, Result};
use rand::Rng;

/// A factory used to build generators for chain graphs.
///
/// Such factories can be created by passing `chain/n` to [`generators::generator_factory_from_str`](crate::generators#generator_factory_from_str),
/// where `n` is the size of chain to produce and must at least 0.
#[derive(Default)]
pub struct ChainGeneratorFactory;

impl<R> NamedParam<BoxedGenerator<R>> for ChainGeneratorFactory {
    fn name(&self) -> &'static str {
        "chain"
    }

    fn description(&self) -> Vec<&'static str> {
        vec![
            "A generator producing a chain of nodes.",
            "The first parameter gives the length of the chain.",
        ]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<R>> {
        let context = "while building a chain generator";
        let int_params = utils::str_param_to_positive_integers(params).context(context)?;
        if let &[n] = int_params.as_slice() {
            Ok(Box::new(move |_| match n {
                0 => Graph::default(),
                1 => {
                    let mut g = Graph::with_capacity(1, 0);
                    g.new_node();
                    g
                }
                _ => {
                    let mut g = Graph::with_capacity(n, n - 1);
                    (0..n - 1).for_each(|i| g.new_edge(i, i + 1));
                    g
                }
            }))
        } else {
            Err(anyhow!("expected exactly 1 parameter")).context(context)
        }
    }
}

impl<R> GeneratorFactory<R> for ChainGeneratorFactory where R: Rng {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeIndexType;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_not_enough_params() {
        assert!(
            (ChainGeneratorFactory.try_with_params("") as Result<BoxedGenerator<ThreadRng>>)
                .is_err()
        )
    }

    #[test]
    fn test_too_much_params() {
        assert!(
            (ChainGeneratorFactory.try_with_params("1,1") as Result<BoxedGenerator<ThreadRng>>)
                .is_err()
        )
    }

    #[test]
    fn test_chain_of_zero() {
        let mut rng = rand::thread_rng();
        let g = ChainGeneratorFactory.try_with_params("0").unwrap()(&mut rng);
        assert_eq!(0, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_chain_of_one() {
        let mut rng = rand::thread_rng();
        let g = ChainGeneratorFactory.try_with_params("1").unwrap()(&mut rng);
        assert_eq!(1, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_chain() {
        let mut rng = rand::thread_rng();
        let g = ChainGeneratorFactory.try_with_params("3").unwrap()(&mut rng);
        assert_eq!(3, g.n_nodes());
        assert_eq!(
            vec![(0, 1), (1, 2)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }
}
