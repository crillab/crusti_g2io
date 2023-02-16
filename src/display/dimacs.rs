use petgraph::EdgeType;

use crate::Graph;
use std::fmt::Display;

impl<Ty> Graph<Ty>
where
    Ty: EdgeType,
{
    /// Returns an object used to display the graph using the [Dimacs format](https://iccma2023.github.io/rules.html#input-format).
    pub fn to_dimacs_display(&self) -> DimacsDisplay<Ty> {
        DimacsDisplay(self)
    }
}

pub struct DimacsDisplay<'a, Ty>(&'a Graph<Ty>)
where
    Ty: EdgeType;

impl<Ty> Display for DimacsDisplay<'_, Ty>
where
    Ty: EdgeType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "p af {}", self.0.n_nodes())?;
        for e in self.0.iter_edges() {
            writeln!(f, "{} {}", e.0 + 1, e.1 + 1)?;
        }
        Ok(())
    }
}
