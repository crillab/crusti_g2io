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
//! let generator = generators::directed_generator_factory_from_str("ba/100,5").unwrap();
//! let mut rng = rand_pcg::Pcg32::from_entropy();
//! // building a graph
//! let g1 = generator(&mut rng);
//! // building another graph with the same generator
//! let g2 = generator(&mut rng);
//! ```
//!
//! # Setting up a new generator factory
//!
//! To create a new generator factory and use it with crusti_g2io, you have to follow these two steps:
//!
//! 1. Write your generator factory in a dedicated `.rs` file in the `src/generators`
//! 2. Register them in the `generators` module
//!
//! In this section, we rewrite the [`PathGeneratorFactory`].
//!
//! The first step in to create the file `src/generators/path_generator.rs`, and register it in `src/generators/mod.rs` by adding the line `mod path_generator;`.
//!
//! Registering it before writing it will let your IDE try to compile it and show you the potential issues in your code.
//! Now, it is time to write the code of the factory in `src/generators/path_generator.rs`.
//!
//! A generator factory in an object with a name, a description and an expected set of parameters, that provides a function (the generator) that generates graphs given a pseudorandom number generator (PRNG).
//! Typically, this kind of object does not need fields; we can so declare it as an empty structure and make it derive the [`Default`] trait.
//! Deriving this trait creates the `default` function which returns a new instance of the factory.
//!
//! ```
//! #[derive(Default)]
//! pub struct PathGeneratorFactoryClone;
//! ```
//!
//! A generator factory is a structure that implements the [`GeneratorFactory`] trait, which itself inherits from [`NamedParam`].
//! Let's write the two trait implementations with their functions left unimplemented.
//!
//! ```
//! # use crusti_g2io::{PathGeneratorFactory, NamedParam, ParameterType, ParameterValue};
//! # use crusti_g2io::generators::{BoxedGenerator, GeneratorFactory};
//! # use petgraph::EdgeType;
//! # use rand::Rng;
//! # use anyhow::Result;
//! # #[derive(Default)]
//! # pub struct PathGeneratorFactoryClone;
//! impl<Ty, R> NamedParam<BoxedGenerator<Ty, R>> for PathGeneratorFactoryClone
//! where
//!     Ty: EdgeType,
//! {
//!     fn name(&self) -> &'static str {
//!         todo!()
//!     }
//!
//!     fn description(&self) -> Vec<&'static str> {
//!         todo!()
//!     }
//!
//!     fn expected_parameter_types(&self) -> Vec<ParameterType> {
//!         todo!()
//!     }
//!
//!     fn try_with_params(
//!         &self,
//!         parameter_values: Vec<ParameterValue>,
//!     ) -> Result<BoxedGenerator<Ty, R>> {
//!         todo!()
//!     }
//! }
//!
//! impl<Ty, R> GeneratorFactory<Ty, R> for PathGeneratorFactoryClone
//! where
//!     R: Rng,
//!     Ty: EdgeType,
//! {
//! }
//! ```
//!
//! First, we can see that [`GeneratorFactory`] does not add functions to implement.
//! We can focus on [`NamedParam`] only.
//!
//! The `name` function must simply return a string that gives the factory name used for the CLI.
//! The `description` function returns a list of strings that are displayed when listing the generator factories using the CLI, one string per line.
//! Their implementations are straightforward.
//!
//! ```
//! # struct Dummy;
//! # impl Dummy {
//!     fn name(&self) -> &'static str {
//!         "path"
//!     }
//!
//!     fn description(&self) -> Vec<&'static str> {
//!         vec![
//!             "A generator producing a path graph.",
//!             "The first parameter gives the length of the path.",
//!         ]
//!     }
//! # }
//! ```
//!
//! The `expected_parameter_types` returns the types of the expected parameters as a vector of [`ParameterType`](crate::ParameterType).
//! For the path generator, a single positive integer (the length of the path) must be provided.
//!
//! ```
//! # use crusti_g2io::ParameterType;
//! # struct Dummy;
//! # impl Dummy {
//!     fn expected_parameter_types(&self) -> Vec<ParameterType> {
//!         vec![ParameterType::PositiveInteger]
//!     }
//! # }
//! ```
//!
//! The only function that is not trivial to implement is `try_with_params`.
//! Do not fear its return type; it is just either en error or a closure allocated on the heap.
//! The closure is the generator: given the context built upon the parameters given through the CLI, it produce a graph from a PRNG.
//!
//! The function `try_with_params` takes as input a vector of parameter values.
//! At this point, the string provided on the CLI has already been parsed and checked against the expected type.
//! You just have to unwrap the parameter values with the correct `unwrap_` function, depending on the parameter type.
//! In the case of the [`PathGeneratorFactory`], we expect parameters to contain a single positive integer; so we can get it with `parameter_values[0].unwrap_usize()`.
//!
//! The generator must produce path graphs of `n` nodes, for any (unused) PRNG.
//! That is, if `n` is 0, the function must return an empty graph.
//! For `n` equal to one, a graph containing a single node must be returned.
//! For higher values of `n`, the graph must contain `n` nodes and edges for each couple of nodes `(i, i+1)`.
//! In the latter case, note that the edge addition is enough to declare the nodes.
//!
//! ```
//! # use crusti_g2io::{Graph, generators::BoxedGenerator};
//! # type Ty = petgraph::Directed;
//! # type R = usize;
//! # fn dummy() -> BoxedGenerator<Ty, R> {
//! # let n = 0;
//! # Box::new(
//! move |prng| match n {
//!     0 => Graph::default(),
//!     1 => {
//!         let mut g = Graph::with_capacity(1, 0);
//!         g.new_node();
//!         g
//!     }
//!     _ => {
//!         let mut g = Graph::with_capacity(n, n - 1);
//!         (0..n - 1).for_each(|i| g.new_edge(i, i + 1));
//!         g
//!     }
//! }
//! # )}
//! ```
//!
//! In this closure, as `prng` is not used, it should be prefixed or replaced by `_`.
//! Finally, to match the return type, the closure must be encapsulated by a [`Box`] (to set it on the heap) and by `Ok` (to translate it into a successful result).
//! Altogether, our function can be written this way:
//!
//! ```
//! # use crusti_g2io::{generators::BoxedGenerator, Graph, ParameterValue};
//! # use anyhow::Result;
//! # type Ty = petgraph::Directed;
//! # type R = usize;
//! # struct Dummy;
//! # impl Dummy {
//! fn try_with_params(
//!     &self,
//!     parameter_values: Vec<ParameterValue>,
//! ) -> Result<BoxedGenerator<Ty, R>> {
//!     let n = parameter_values[0].unwrap_usize();
//!     Ok(Box::new(move |_| match n {
//!         0 => Graph::default(),
//!         1 => {
//!             let mut g = Graph::with_capacity(1, 0);
//!             g.new_node();
//!             g
//!         }
//!         _ => {
//!             let mut g = Graph::with_capacity(n, n - 1);
//!             (0..n - 1).for_each(|i| g.new_edge(i, i + 1));
//!             g
//!         }
//!     }))
//! }
//! # }
//! ```
//!
//! Interestingly, this generator is able to produce both undirected and directed graphs.
//! The only difference is the semantics of the [`new_edge`](crate::Graph#method.new_edge) function, which produce a an undirected or a directed edge, depending on the context.
//! In case the kind of graph is important, one can check the Boolean value returned by `Ty::is_directed()`.
//!
//! Finally, the last step is to register the generator factory into `src/generators/mod.rs`.
//! To do this, import the factory into `mod.rs` with a statement `pub use path_generator::PathGeneratorFactory;`,
//! and add it to the set of undirected and directed factories:
//!
//! ```
//! # use lazy_static::lazy_static;
//! # use crusti_g2io::{generators::GeneratorFactory, BarabasiAlbertGeneratorFactory, ErdosRenyiGeneratorFactory, TreeGeneratorFactory, WattsStrogatzGeneratorFactory, PathGeneratorFactory};
//! # use petgraph::{Directed, Undirected};
//! # use rand_pcg::Pcg32;
//! # type PathGeneratorFactoryClone = PathGeneratorFactory;
//! lazy_static! {
//!     pub(crate) static ref GENERATOR_FACTORIES_DIRECTED_PCG32: [Box<dyn GeneratorFactory<Directed, Pcg32> + Sync>; 5] = [
//!         Box::new(BarabasiAlbertGeneratorFactory::default()),
//!         Box::new(ErdosRenyiGeneratorFactory::default()),
//!         Box::new(TreeGeneratorFactory::default()),
//!         Box::new(WattsStrogatzGeneratorFactory::default()),
//!         Box::new(PathGeneratorFactoryClone::default()),
//!     ];
//! }
//!
//! lazy_static! {
//!     pub(crate) static ref GENERATOR_FACTORIES_UNDIRECTED_PCG32: [Box<dyn GeneratorFactory<Undirected, Pcg32> + Sync>; 5] = [
//!         Box::new(BarabasiAlbertGeneratorFactory::default()),
//!         Box::new(ErdosRenyiGeneratorFactory::default()),
//!         Box::new(TreeGeneratorFactory::default()),
//!         Box::new(WattsStrogatzGeneratorFactory::default()),
//!         Box::new(PathGeneratorFactoryClone::default()),
//!     ];
//! }
//! ```

mod barabasi_albert_generator;
pub use barabasi_albert_generator::BarabasiAlbertGeneratorFactory;

mod path_generator;
pub use path_generator::PathGeneratorFactory;

mod erdos_renyi;
pub use erdos_renyi::ErdosRenyiGeneratorFactory;

mod tree_generator;
pub use tree_generator::TreeGeneratorFactory;

mod watts_strogatz;
pub use watts_strogatz::WattsStrogatzGeneratorFactory;

use crate::{core::named_param, Graph, NamedParam};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use petgraph::{Directed, EdgeType, Undirected};
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
/// let generator = generators::directed_generator_factory_from_str("path/3").unwrap();
/// let graph = generator(&mut rand_pcg::Pcg32::from_entropy());
/// ```
pub type BoxedGenerator<Ty, R> = Box<dyn Fn(&mut R) -> Graph<Ty> + Sync + Send>;

/// A trait for objects that produce graph generators.
pub trait GeneratorFactory<Ty, R>: NamedParam<BoxedGenerator<Ty, R>>
where
    R: Rng,
    Ty: EdgeType,
{
}

lazy_static! {
    pub(crate) static ref GENERATOR_FACTORIES_DIRECTED_PCG32: [Box<dyn GeneratorFactory<Directed, Pcg32> + Sync>; 5] = [
        Box::new(BarabasiAlbertGeneratorFactory::default()),
        Box::new(PathGeneratorFactory::default()),
        Box::new(ErdosRenyiGeneratorFactory::default()),
        Box::new(TreeGeneratorFactory::default()),
        Box::new(WattsStrogatzGeneratorFactory::default()),
    ];
}

lazy_static! {
    pub(crate) static ref GENERATOR_FACTORIES_UNDIRECTED_PCG32: [Box<dyn GeneratorFactory<Undirected, Pcg32> + Sync>; 5] = [
        Box::new(BarabasiAlbertGeneratorFactory::default()),
        Box::new(PathGeneratorFactory::default()),
        Box::new(ErdosRenyiGeneratorFactory::default()),
        Box::new(TreeGeneratorFactory::default()),
        Box::new(WattsStrogatzGeneratorFactory::default()),
    ];
}

/// Iterates over all the directed graph generator factories.
///
/// ```
/// # use crusti_g2io::generators;
/// generators::iter_directed_generator_factories().enumerate().for_each(|(i,g)| {
///     println!(r#"generator {} has name "{}""#, i, g.name());
/// });
/// ```
pub fn iter_directed_generator_factories(
) -> impl Iterator<Item = &'static (dyn GeneratorFactory<Directed, Pcg32> + Sync + 'static)> + 'static
{
    GENERATOR_FACTORIES_DIRECTED_PCG32
        .iter()
        .map(|b| b.as_ref())
}

/// Iterates over all the undirected graph generator factories.
///
/// ```
/// # use crusti_g2io::generators;
/// generators::iter_undirected_generator_factories().enumerate().for_each(|(i,g)| {
///     println!(r#"generator {} has name "{}""#, i, g.name());
/// });
/// ```
pub fn iter_undirected_generator_factories(
) -> impl Iterator<Item = &'static (dyn GeneratorFactory<Undirected, Pcg32> + Sync + 'static)> + 'static
{
    GENERATOR_FACTORIES_UNDIRECTED_PCG32
        .iter()
        .map(|b| b.as_ref())
}

/// Given a string representing a parameterized directed graph generator factory, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::generators;
/// assert!(generators::directed_generator_factory_from_str("path/3").is_ok()); // OK
/// assert!(generators::directed_generator_factory_from_str("path/1,2,3").is_err()); // wrong parameters
/// assert!(generators::directed_generator_factory_from_str("foo/3").is_err()); // unknown generator
/// ```
pub fn directed_generator_factory_from_str(s: &str) -> Result<BoxedGenerator<Directed, Pcg32>> {
    named_param::named_from_str(GENERATOR_FACTORIES_DIRECTED_PCG32.as_slice(), s)
        .context("while building a generator from a string")
}

/// Given a string representing a parameterized undirected graph generator factory, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::generators;
/// assert!(generators::undirected_generator_factory_from_str("path/3").is_ok()); // OK
/// assert!(generators::undirected_generator_factory_from_str("path/1,2,3").is_err()); // wrong parameters
/// assert!(generators::undirected_generator_factory_from_str("foo/3").is_err()); // unknown generator
/// ```
pub fn undirected_generator_factory_from_str(s: &str) -> Result<BoxedGenerator<Undirected, Pcg32>> {
    named_param::named_from_str(GENERATOR_FACTORIES_UNDIRECTED_PCG32.as_slice(), s)
        .context("while building a generator from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_ok() {
        assert!(directed_generator_factory_from_str("path/1").is_ok());
    }

    #[test]
    fn test_unknown_generator() {
        assert!(directed_generator_factory_from_str("foo/1").is_err());
    }

    #[test]
    fn test_generator_no_params() {
        assert!(directed_generator_factory_from_str("path").is_err());
    }

    #[test]
    fn test_generator_too_much_params() {
        assert!(directed_generator_factory_from_str("path/1,1").is_err());
    }

    #[test]
    fn test_generator_wrong_types_params() {
        assert!(directed_generator_factory_from_str("path/0.5").is_err());
    }
}
