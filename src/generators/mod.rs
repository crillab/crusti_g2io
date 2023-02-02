mod barabasi_albert_generator;
pub use barabasi_albert_generator::BarabasiAlbertGeneratorFactory;

mod chain_generator;
pub use chain_generator::ChainGeneratorFactory;

use crate::{
    graph::Graph,
    utils::{self, Named},
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, Rng};

pub type BoxedGenerator<R> = Box<dyn Fn(&mut R) -> Graph>;

pub trait GeneratorFactory<R>: Named<BoxedGenerator<R>>
where
    R: Rng,
{
}

lazy_static! {
    pub(crate) static ref FACTORIES_THREAD_RNG: [Box<dyn GeneratorFactory<ThreadRng> + Sync>; 2] = [
        Box::new(BarabasiAlbertGeneratorFactory),
        Box::new(ChainGeneratorFactory)
    ];
}

pub fn generator_factory_from_str(s: &str) -> Result<BoxedGenerator<ThreadRng>> {
    utils::named_from_str(FACTORIES_THREAD_RNG.as_slice(), s)
        .context("while building a generator from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_ok() {
        assert!(generator_factory_from_str("chain/1").is_ok());
    }

    #[test]
    fn test_unknown_generator() {
        assert!(generator_factory_from_str("foo/1").is_err());
    }

    #[test]
    fn test_generator_no_params() {
        assert!(generator_factory_from_str("chain").is_err());
    }
}
