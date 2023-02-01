mod barabasi_albert_generator;
mod chain_generator;

use crate::graph::Graph;
use anyhow::{anyhow, Context, Result};
use barabasi_albert_generator::BarabasiAlbertGeneratorFactory;
use chain_generator::ChainGeneratorFactory;
use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, Rng};

pub type BoxedGenerator<R> = Box<dyn Fn(&mut R) -> Graph>;

pub trait GeneratorFactory<R>
where
    R: Rng,
{
    fn name(&self) -> &'static str;

    fn try_with_params(&self, params: &str) -> Result<BoxedGenerator<R>>;
}

lazy_static! {
    static ref FACTORIES_THREAD_RNG: [Box<dyn GeneratorFactory<ThreadRng> + Sync>; 2] = [
        Box::new(BarabasiAlbertGeneratorFactory),
        Box::new(ChainGeneratorFactory)
    ];
}

pub fn generator_factory_from_str(s: &str) -> Result<BoxedGenerator<ThreadRng>> {
    let context = "while building a generator from a string";
    let make_err = || anyhow!(r#"cannot build a generator from the string "{}""#, s);
    let (kind, str_params) = s.split_once('/').ok_or_else(make_err)?;
    for factory in FACTORIES_THREAD_RNG.iter() {
        if factory.name() == kind {
            return factory.try_with_params(str_params).context(context);
        }
    }
    Err(anyhow!(r#"unknown generator "{}""#, s)).context(context)
}

fn str_param_to_positive_integers(str_params: &str) -> Result<Vec<usize>> {
    if str_params.is_empty() {
        return Ok(vec![]);
    }
    str_params
        .split(',')
        .map(|i| {
            str::parse::<usize>(i)
                .context("while translating a string into a vector of positive integers")
        })
        .collect::<Result<Vec<usize>>>()
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

    #[test]
    fn test_str_param_to_positive_integers_ok_1() {
        assert_eq!(vec![1], str_param_to_positive_integers("1").unwrap())
    }

    #[test]
    fn test_str_param_to_positive_integers_ok_2() {
        assert_eq!(vec![1, 2], str_param_to_positive_integers("1,2").unwrap())
    }

    #[test]
    fn test_str_param_to_positive_integers_empty() {
        assert_eq!(
            vec![] as Vec<usize>,
            str_param_to_positive_integers("").unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integers_single_comma() {
        assert!(str_param_to_positive_integers(",").is_err())
    }

    #[test]
    fn test_str_param_to_positive_integers_trailing_comma() {
        assert!(str_param_to_positive_integers("1,2,").is_err())
    }

    #[test]
    fn test_str_param_to_positive_integers_alpha() {
        assert!(str_param_to_positive_integers("a").is_err())
    }
}
