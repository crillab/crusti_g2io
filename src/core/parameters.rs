use anyhow::{anyhow, Context, Result};

pub(crate) struct ParameterParser {
    parameter_types: Vec<ParameterType>,
}

impl ParameterParser {
    pub fn new(parameter_types: Vec<ParameterType>) -> Self {
        Self { parameter_types }
    }

    pub fn parse(&self, str_params: &str) -> Result<Vec<ParameterValue>> {
        let parameters = if str_params.is_empty() {
            vec![]
        } else {
            str_params.split(',').collect::<Vec<&str>>()
        };
        if parameters.len() != self.parameter_types.len() {
            return Err(anyhow!(
                "expected {} parameters, got {}",
                self.parameter_types.len(),
                parameters.len()
            ));
        }
        (0..parameters.len())
            .map(|i| self.parameter_types[i].parse(parameters[i]))
            .collect()
    }
}

/// The types allowed for parameters.
pub enum ParameterType {
    /// A positive integer, possibly null
    PositiveInteger,
    /// A floating point number between 0 and 1 (both allowed)
    Probability,
}

impl ParameterType {
    fn parse(&self, param: &str) -> Result<ParameterValue> {
        Ok(match self {
            ParameterType::PositiveInteger => ParameterValue::PositiveInteger(
                str::parse::<usize>(param)
                    .context("while translating a string into a positive integer")?,
            ),
            ParameterType::Probability => {
                let context = "while translating a string into a probability";
                let p = str::parse(param).context(context)?;
                if !(0. ..=1.).contains(&p) {
                    return Err(anyhow!("probability must be between 0 and 1")).context(context);
                } else {
                    ParameterValue::Probability(p)
                }
            }
        })
    }
}

/// A wrapper around a value read from a string.
///
/// Its value should have been checked against an unexpected type.
/// The value must be unwrapped using a compatible function.
#[derive(Debug, PartialEq)]
pub enum ParameterValue {
    /// A positive integer, possibly null
    PositiveInteger(usize),
    /// A floating point number between 0 and 1 (both allowed)
    Probability(f64),
}

impl ParameterValue {
    /// Unwraps a parameter value which value can be seen as a positive integer.
    ///
    /// # Panics
    ///
    /// This function panics if the value can not be seen as a positive integer.
    pub fn unwrap_usize(&self) -> usize {
        match self {
            ParameterValue::PositiveInteger(n) => *n,
            _ => panic!(),
        }
    }

    /// Unwraps a parameter value which value can be seen as a probability.
    ///
    /// # Panics
    ///
    /// This function panics if the value can not be seen as a probability.
    pub fn unwrap_f64(&self) -> f64 {
        match self {
            ParameterValue::Probability(f) => *f,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_not_enough_params_0() {
        let parser = ParameterParser::new(vec![
            ParameterType::PositiveInteger,
            ParameterType::PositiveInteger,
        ]);
        assert!(parser.parse("").is_err());
    }

    #[test]
    pub fn test_not_enough_params_1() {
        let parser = ParameterParser::new(vec![
            ParameterType::PositiveInteger,
            ParameterType::PositiveInteger,
        ]);
        assert!(parser.parse("1").is_err());
    }

    #[test]
    pub fn test_ok_empty() {
        let parser = ParameterParser::new(vec![]);
        assert_eq!(vec![] as Vec<ParameterValue>, parser.parse("").unwrap());
    }

    #[test]
    pub fn test_positive_integer_ok() {
        let parser = ParameterParser::new(vec![ParameterType::PositiveInteger]);
        assert_eq!(
            vec![ParameterValue::PositiveInteger(1)],
            parser.parse("1").unwrap()
        );
    }

    #[test]
    pub fn test_positive_integer_not_ok() {
        let parser = ParameterParser::new(vec![ParameterType::PositiveInteger]);
        assert!(parser.parse("-1").is_err());
        assert!(parser.parse("a").is_err());
    }

    #[test]
    pub fn test_probability_ok() {
        let parser = ParameterParser::new(vec![ParameterType::Probability]);
        assert_eq!(
            vec![ParameterValue::Probability(0.5)],
            parser.parse(".5").unwrap()
        );
        assert_eq!(
            vec![ParameterValue::Probability(0.)],
            parser.parse("0").unwrap()
        );
        assert_eq!(
            vec![ParameterValue::Probability(1.)],
            parser.parse("1").unwrap()
        );
    }

    #[test]
    pub fn test_probability_not_ok() {
        let parser = ParameterParser::new(vec![ParameterType::Probability]);
        assert!(parser.parse("-1.5").is_err());
        assert!(parser.parse("1.5").is_err());
        assert!(parser.parse("a").is_err());
    }
}
