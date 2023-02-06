use anyhow::{anyhow, Context, Result};

pub(crate) fn str_param_to_positive_integers(str_params: &str) -> Result<Vec<usize>> {
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

pub(crate) fn str_param_to_positive_integer_and_probability(
    str_params: &str,
) -> Result<(usize, f64)> {
    let words = str_params.split(',').collect::<Vec<&str>>();
    match words.as_slice() {
        [n, p] => {
            let context_p = "while parsing the probability";
            let probability = str::parse(p).context(context_p)?;
            if !(0. ..=1.).contains(&probability) {
                Err(anyhow!("probability must be between 0 and 1")).context(context_p)
            } else {
                Ok((
                    str::parse(n).context("while parsing the positive integer")?,
                    probability,
                ))
            }
        }
        _ => Err(anyhow!("expected exactly 2 parameters")),
    }
    .context("while translating a string into a a positive integer and a probability")
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

    #[test]
    fn test_str_param_to_positive_integer_and_probability_ok_1() {
        assert_eq!(
            (0, 0.),
            str_param_to_positive_integer_and_probability("0,0").unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_ok_2() {
        assert_eq!(
            (0, 0.5),
            str_param_to_positive_integer_and_probability("0,0.5").unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_empty() {
        assert!(str_param_to_positive_integer_and_probability("").is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_single_comma() {
        assert!(str_param_to_positive_integer_and_probability(",").is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_not_an_integer() {
        assert!(str_param_to_positive_integer_and_probability("0.,0.").is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_not_a_float() {
        assert!(str_param_to_positive_integer_and_probability("0.,a").is_err())
    }
}
