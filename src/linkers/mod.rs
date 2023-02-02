mod first_to_first;
pub use first_to_first::FirstToFirstLinker;

use crate::{
    graph::{Graph, InterGraphEdge},
    utils::{self, Named},
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;

pub type BoxedLinker = Box<dyn Fn(&Graph, &Graph) -> Vec<InterGraphEdge>>;

pub trait Linker: Named<BoxedLinker> {}

lazy_static! {
    static ref LINKERS: [Box<dyn Linker + Sync>; 1] = [Box::new(FirstToFirstLinker),];
}

pub fn linker_from_str(s: &str) -> Result<BoxedLinker> {
    utils::named_from_str(LINKERS.as_slice(), s).context("while building a linker from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_ok() {
        assert!(linker_from_str("f2f").is_ok());
    }

    #[test]
    fn test_unknown_linker() {
        assert!(linker_from_str("foo/1").is_err());
    }

    #[test]
    fn test_linker_too_much_params() {
        assert!(linker_from_str("f2f/1").is_err());
    }
}
