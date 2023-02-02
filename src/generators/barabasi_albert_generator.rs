use super::{BoxedGenerator, GeneratorFactory};
use crate::utils::{self, Named};
use anyhow::{anyhow, Context, Result};
use petgraph_gen::barabasi_albert_graph;
use rand::Rng;

pub struct BarabasiAlbertGeneratorFactory;

impl<R> Named<BoxedGenerator<R>> for BarabasiAlbertGeneratorFactory
where
    R: Rng,
{
    fn name(&self) -> &'static str {
        "ba"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["A generator following the Barabási-Albert model, initialized by a star graph.","First parameter gives the number of nodes of the graph, while the second one gives the number of nodes of the initial star graph."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<R>> {
        let context = "while building a Barabasi-Albert generator";
        let int_params = utils::str_param_to_positive_integers(params).context(context)?;
        if let &[n, m] = int_params.as_slice() {
            if m == 0 || m >= n {
                return Err(anyhow!(
                    r#"second parameter ("m") must be higher than 0 and lower than the first one ("n")"#
                )).context(context);
            }
            Ok(Box::new(move |r| {
                barabasi_albert_graph(r, n, m, None).into()
            }))
        } else {
            Err(anyhow!("expected exactly 2 parameters")).context(context)
        }
    }
}

impl<R> GeneratorFactory<R> for BarabasiAlbertGeneratorFactory where R: Rng {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeIndexType;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_not_enough_params() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params("1")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_too_much_params() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params("2,1,0")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_m_is_zero() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params("2,0")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_n_is_not_higher_than_m() {
        assert!((BarabasiAlbertGeneratorFactory.try_with_params("2,2")
            as Result<BoxedGenerator<ThreadRng>>)
            .is_err())
    }

    #[test]
    fn test_barabasi_star() {
        let mut rng = rand::thread_rng();
        let g = BarabasiAlbertGeneratorFactory
            .try_with_params("4,3")
            .unwrap()(&mut rng);
        assert_eq!(
            vec![(0, 1), (0, 2), (0, 3)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }
}
