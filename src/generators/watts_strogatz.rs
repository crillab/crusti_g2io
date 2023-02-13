use super::{BoxedGenerator, GeneratorFactory};
use crate::{core::utils, Graph, NamedParam};
use anyhow::{anyhow, Context, Result};
use rand::{distributions::Uniform, prelude::Distribution, Rng};

/// A factory used to build generators for graphs following the [Watts-Strogatz](https://en.wikipedia.org/wiki/Watts%E2%80%93Strogatz_model) model.
///
/// Such factories can be created by passing `ws/n,k,p` to [`generators::generator_factory_from_str`](crate::generators#generator_factory_from_str) where
///   - `n` is the size of graph to produce;
///   - `k` is the (even) degree of the nodes of the initial ring lattice;
///   - `p` is the edge rewire probability.
///
/// Parameter `k` must be positive, `n` must be strictly greater than `k` and `p` must be between 0 and 1.
#[derive(Default)]
pub struct WattsStrogatzGeneratorFactory;

impl<R> NamedParam<BoxedGenerator<R>> for WattsStrogatzGeneratorFactory
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "ws"
    }

    fn description(&self) -> Vec<&'static str> {
        vec![
            "A generator following the Watts-Strogatz model.",
            "First parameter gives the number of nodes, the second one gives the initial node degree and the third is the rewire probability."
        ]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<R>> {
        let context = "while building a Watts-Strogatz generator";
        let (v, p) =
            utils::str_param_to_positive_integers_and_probability(params, 2).context(context)?;
        let n = v[0];
        let k = v[1];
        if k & 1 == 1 {
            return Err(anyhow!(r#"second parameter ("k") must be even"#)).context(context);
        }
        if n <= k {
            return Err(anyhow!(
                r#"first parameter ("n") must be higher than the second one ("k")"#
            ))
            .context(context);
        }
        Ok(Box::new(move |r| build_graph(n, k, p, r)))
    }
}

fn build_graph<R>(n: usize, k: usize, p: f64, r: &mut R) -> Graph
where
    R: Rng,
{
    let mut g = Graph::with_capacity(n, n * k);
    (0..n).for_each(|i| {
        (0..k / 2).for_each(|j| {
            g.new_edge(i, (i + j + 1) % n);
        });
    });
    let proba_uniform = Uniform::new_inclusive(0., 1.);
    let index_uniform = Uniform::new(0, n - 1 - k / 2);
    (0..n).for_each(|i| {
        let last_target = (i + 1 + k / 2) % n;
        let mut not_targets: Vec<usize> = if i < last_target {
            (0..i).chain(i + 1 + k / 2..n).collect()
        } else {
            ((i + 1 + k / 2) % n..i).collect()
        };
        (0..k / 2).for_each(|j| {
            if proba_uniform.sample(r) < p {
                let index = index_uniform.sample(r);
                let new_target = not_targets[index];
                let old_target = (i + 1 + j) % n;
                not_targets[index] = old_target;
                g.remove_edge(i, old_target);
                g.new_edge(i, new_target);
            }
        });
    });
    g
}

impl<R> GeneratorFactory<R> for WattsStrogatzGeneratorFactory where R: Rng {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeIndexType;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_not_enough_params() {
        assert!((WattsStrogatzGeneratorFactory.try_with_params("3, 2")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_too_much_params() {
        assert!((WattsStrogatzGeneratorFactory.try_with_params("3, 2,0,0.5")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_k_is_not_even() {
        assert!((WattsStrogatzGeneratorFactory.try_with_params("3, 1, 0.5")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_n_is_not_higher_than_k() {
        assert!((WattsStrogatzGeneratorFactory.try_with_params("2,2,0.5")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_p_is_zero_5_4() {
        let mut rng = rand::thread_rng();
        let g = WattsStrogatzGeneratorFactory
            .try_with_params("5,4,0")
            .unwrap()(&mut rng);
        let mut edges = g
            .iter_edges()
            .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
        edges.sort_unstable();
        assert_eq!(
            vec![
                (0, 1),
                (0, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (2, 4),
                (3, 0),
                (3, 4),
                (4, 0),
                (4, 1)
            ],
            edges
        );
    }

    #[test]
    fn test_p_is_zero_3_2() {
        let mut rng = rand::thread_rng();
        let g = WattsStrogatzGeneratorFactory
            .try_with_params("3,2,0")
            .unwrap()(&mut rng);
        let mut edges = g
            .iter_edges()
            .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
        edges.sort_unstable();
        assert_eq!(vec![(0, 1), (1, 2), (2, 0),], edges);
    }

    #[test]
    fn test_p_is_one_3_2() {
        let mut rng = rand::thread_rng();
        let g = WattsStrogatzGeneratorFactory
            .try_with_params("3,2,1")
            .unwrap()(&mut rng);
        let mut edges = g
            .iter_edges()
            .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
        edges.sort_unstable();
        assert_eq!(vec![(0, 2), (1, 0), (2, 1),], edges);
    }
}