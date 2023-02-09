use crate::Graph;
use std::fmt::Display;

impl Graph {
    /// Returns an object used to display the graph using the [Dimacs format](https://iccma2023.github.io/rules.html#input-format).
    pub fn to_dimacs_display(&self) -> DimacsDisplay {
        DimacsDisplay(self)
    }
}

pub struct DimacsDisplay<'a>(&'a Graph);

impl Display for DimacsDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dimacs_display = (write("coucou"));
        std::fmt::Debug::fmt(&dimacs_display, f)
    }
}
