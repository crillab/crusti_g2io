use anyhow::{anyhow, Context, Result};

pub trait Named<T> {
    fn name(&self) -> &'static str;

    fn try_with_params(&self, params: &str) -> Result<T>;
}

pub fn named_from_str<T, S>(collection: &[Box<S>], s: &str) -> Result<T>
where
    S: Named<T> + Sync + ?Sized,
{
    let context = "while building a named object from a string";
    let (kind, str_params) = match s.split_once('/') {
        Some((k, p)) => (k, p),
        None => (s, ""),
    };
    for named_factory in collection.iter() {
        if named_factory.name() == kind {
            return named_factory.try_with_params(str_params).context(context);
        }
    }
    Err(anyhow!(r#"unknown named object "{}""#, s)).context(context)
}

pub fn str_param_to_positive_integers(str_params: &str) -> Result<Vec<usize>> {
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
