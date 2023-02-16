use super::{BoxedGenerator, GeneratorFactory};
use crate::{core::utils, Graph, NamedParam};
use anyhow::{anyhow, Context, Result};
use petgraph::EdgeType;
use rand::Rng;

/// A factory used to build generators for balanced trees.
///
/// Such factories can be created by passing `tree/n` to [`generators::generator_factory_from_str`](crate::generators#generator_factory_from_str),
/// where `n` is the number of node and must be at least 1.
#[derive(Default)]
pub struct TreeGeneratorFactory;

impl<Ty, R> NamedParam<BoxedGenerator<Ty, R>> for TreeGeneratorFactory
where
    Ty: EdgeType,
{
    fn name(&self) -> &'static str {
        "tree"
    }

    fn description(&self) -> Vec<&'static str> {
        vec![
            "A generator producing a tree.",
            "The first parameter gives the number of nodes.",
            "The tree is well balanced.",
        ]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<Ty, R>> {
        let context = "while building a tree generator";
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
                    let mut g = Graph::with_capacity(n, 0);
                    (0..n).for_each(|i| {
                        if 2 * i + 1 < n {
                            g.new_edge(i, 2 * i + 1);
                        }

                        if 2 * i + 2 < n {
                            g.new_edge(i, 2 * i + 2);
                        }
                    });
                    g
                }
            }))
        } else {
            Err(anyhow!("expected exactly 1 parameter")).context(context)
        }
    }
}

impl<Ty, R> GeneratorFactory<Ty, R> for TreeGeneratorFactory
where
    R: Rng,
    Ty: EdgeType,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeIndexType;
    use petgraph::Directed;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_not_enough_params() {
        assert!((TreeGeneratorFactory.try_with_params("")
            as Result<BoxedGenerator<Directed, ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_too_much_params() {
        assert!((TreeGeneratorFactory.try_with_params("1,1")
            as Result<BoxedGenerator<Directed, ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_tree_of_zero() {
        let mut rng = rand::thread_rng();
        let g: Graph<Directed> = TreeGeneratorFactory.try_with_params("0").unwrap()(&mut rng);
        assert_eq!(0, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_tree_of_one() {
        let mut rng = rand::thread_rng();
        let g: Graph<Directed> = TreeGeneratorFactory.try_with_params("1").unwrap()(&mut rng);
        assert_eq!(1, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_tree() {
        let mut rng = rand::thread_rng();
        let g: Graph<Directed> = TreeGeneratorFactory.try_with_params("4").unwrap()(&mut rng);
        assert_eq!(4, g.n_nodes());
        assert_eq!(
            vec![(0, 1), (0, 2), (1, 3)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }
}
