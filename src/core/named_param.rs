use crate::{ParameterType, ParameterValue};
use anyhow::{anyhow, Context, Result};

use super::parameters::ParameterParser;

/// A trait for CLI parameters which available values are named alternatives, eg. generators and linkers.
///
/// This trait provides methods to get information on the alternative itself (the name of the parameter, its description),
/// and a method used to build instances of it with some string parameters.
///
/// See [`ChainGeneratorFactory`](crate::ChainGeneratorFactory) source code for a straightforward implementation of this trait.
///
/// ```
/// use crusti_g2io::{generators, ParameterValue};
/// use rand_core::SeedableRng;
///
/// // displaying the available generators
/// for g in generators::iter_directed_generator_factories() {
///     println!(
///         r#"generator "{}" has description {:?}"#,
///         g.name(),
///         g.description(),
///     )
/// }
///
/// // building a generator for chain graphs of length 3 and a related graph.
/// let generator_factory = generators::iter_directed_generator_factories()
///     .find(|f| f.name() == "chain")
///     .unwrap();
/// let generator = generator_factory.try_with_params(vec![ParameterValue::PositiveInteger(3)]).unwrap();
/// let graph = generator(&mut rand_pcg::Pcg32::from_entropy());
/// ```
pub trait NamedParam<T> {
    /// Returns the named associated with the alternative.
    fn name(&self) -> &'static str;

    /// Returns the description associated with the alternative.
    ///
    /// The description is returned as a vector of strings, each containing a logical portion of the description.
    fn description(&self) -> Vec<&'static str>;

    /// Returns the types of the expected parameters.
    ///
    /// In case this objects expect no parameter, this function must return an empty vector.
    fn expected_parameter_types(&self) -> Vec<ParameterType>;

    /// Tries to build an instance of the related alternative given the parameters.
    ///
    /// The parameter must be computed from the expected types returned by `expected_parameter_types` and a string that concatenate the parameters values split by commas.
    fn try_with_params(&self, parameter_values: Vec<ParameterValue>) -> Result<T>;
}

pub(crate) fn named_from_str<T, S>(collection: &[Box<S>], s: &str) -> Result<T>
where
    S: NamedParam<T> + Sync + ?Sized,
{
    let context = "while building a named object from a string";
    let (kind, str_params) = match s.split_once('/') {
        Some((k, p)) => (k, p),
        None => (s, ""),
    };
    for named_factory in collection.iter() {
        if named_factory.name() == kind {
            let parameter_parser = ParameterParser::new(named_factory.expected_parameter_types());
            let parameter_values = parameter_parser
                .parse(str_params)
                .with_context(|| {
                    format!(
                        r#"while evaluating the parameters for a "{}" generator"#,
                        kind
                    )
                })
                .context(context)?;
            return named_factory
                .try_with_params(parameter_values)
                .context(context);
        }
    }
    Err(anyhow!(r#"unknown named object "{}""#, s)).context(context)
}
