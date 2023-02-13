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

pub(crate) fn str_param_to_positive_integers_and_probability(
    str_params: &str,
    n_integers: usize,
) -> Result<(Vec<usize>, f64)> {
    let add_context = |r: Result<(Vec<usize>, f64)>| match n_integers {
        0 => r.context("while formatting a string into a probability".to_string()),
        1 => r.context("while formatting a string into a positive integer and a probability"),
        n => r.with_context(|| {
            format!(
                "while formatting a string into {} positive integers and a probability",
                n
            )
        }),
    };
    let words = str_params.split(',').collect::<Vec<&str>>();
    if words.len() != 1 + n_integers {
        return add_context(Err(anyhow!(
            "expected exactly {} parameters",
            1 + n_integers
        )));
    }
    let integers = words
        .iter()
        .take(n_integers)
        .map(|i| {
            str::parse::<usize>(i).context("while translating a string into a positive integer")
        })
        .collect::<Result<Vec<usize>>>()?;
    let context_p = "while parsing the probability";
    let probability = str::parse(words[n_integers]).context(context_p)?;
    if !(0. ..=1.).contains(&probability) {
        Err(anyhow!("probability must be between 0 and 1")).context(context_p)
    } else {
        Ok((integers, probability))
    }
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
            (vec![0], 0.),
            str_param_to_positive_integers_and_probability("0,0", 1).unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_ok_2() {
        assert_eq!(
            (vec![0], 0.5),
            str_param_to_positive_integers_and_probability("0,0.5", 1).unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_empty() {
        assert!(str_param_to_positive_integers_and_probability("", 1).is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_single_comma() {
        assert!(str_param_to_positive_integers_and_probability(",", 1).is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_not_an_integer() {
        assert!(str_param_to_positive_integers_and_probability("0.,0.", 1).is_err())
    }

    #[test]
    fn test_str_param_to_positive_integer_and_probability_not_a_float() {
        assert!(str_param_to_positive_integers_and_probability("0.,a", 1).is_err())
    }

    #[test]
    fn test_str_param_to_positive_integers_and_probability_single_probability_ok() {
        assert_eq!(
            (vec![], 0.5),
            str_param_to_positive_integers_and_probability("0.5", 0).unwrap()
        )
    }

    #[test]
    fn test_str_param_to_positive_integers_and_probability_single_probability_too_much_integers() {
        assert!(str_param_to_positive_integers_and_probability("1,0.5", 0).is_err())
    }
}
