//! A module dedicated to graph generators.
//!
//! Graph generators are responsible for the creation of both inner and outer graphs.
//! Invoking a graph generator with a random generator produces a graph.
//!
//! ```
//! # use crusti_g2io::generators;
//! use rand_core::SeedableRng;
//!
//! // building a generator for Barabási-Albert graphs.
//! let generator = generators::generator_factory_from_str("ba/100,5").unwrap();
//! let mut rng = rand_pcg::Pcg32::from_entropy();
//! // building a graph
//! let g1 = generator(&mut rng);
//! // building another graph with the same generator
//! let g2 = generator(&mut rng);
//! ```

mod barabasi_albert_generator;
pub use barabasi_albert_generator::BarabasiAlbertGeneratorFactory;

mod chain_generator;
pub use chain_generator::ChainGeneratorFactory;

use crate::{
    graph::Graph,
    utils::{self, NamedParam},
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use rand::Rng;
use rand_pcg::Pcg32;

/// A boxed function that takes a random generator and outputs a graph.
///
/// Such functions are returned by generator factories, and allow the instantiation of graphs.
/// The parameterized type is the one of the random generator.
///
/// ```
/// # use crusti_g2io::generators;
/// use rand_core::SeedableRng;
///
/// // getting a boxed generating function from a string
/// let generator = generators::generator_factory_from_str("chain/3").unwrap();
/// let graph = generator(&mut rand_pcg::Pcg32::from_entropy());
/// ```
pub type BoxedGenerator<R> = Box<dyn Fn(&mut R) -> Graph>;

/// A trait for objects that produce graph generators.
pub trait GeneratorFactory<R>: NamedParam<BoxedGenerator<R>>
where
    R: Rng,
{
}

lazy_static! {
    pub(crate) static ref FACTORIES_THREAD_PCG32: [Box<dyn GeneratorFactory<Pcg32> + Sync>; 2] = [
        Box::new(BarabasiAlbertGeneratorFactory),
        Box::new(ChainGeneratorFactory)
    ];
}

/// Iterates over all the graph generator factories.
///
/// ```
/// # use crusti_g2io::generators;
/// generators::iter_generator_factories().enumerate().for_each(|(i,g)| {
///     println!(r#"generator {} has name "{}""#, i, g.name());
/// });
/// ```
pub fn iter_generator_factories(
) -> impl Iterator<Item = &'static (dyn GeneratorFactory<Pcg32> + Sync + 'static)> + 'static {
    FACTORIES_THREAD_PCG32.iter().map(|b| b.as_ref())
}

/// Given a string representing a parameterized generator factory, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::generators;
/// assert!(generators::generator_factory_from_str("chain/3").is_ok()); // OK
/// assert!(generators::generator_factory_from_str("chain/1,2,3").is_err()); // wrong parameters
/// assert!(generators::generator_factory_from_str("foo/3").is_err()); // unknown generator
/// ```
pub fn generator_factory_from_str(s: &str) -> Result<BoxedGenerator<Pcg32>> {
    utils::named_from_str(FACTORIES_THREAD_PCG32.as_slice(), s)
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
