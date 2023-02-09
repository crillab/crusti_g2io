use super::{BoxedLinker, Linker};
use crate::{Graph, InterGraphEdge, NamedParam};
use anyhow::{anyhow, Context, Result};

/// A linker that connects graph by targeting their nodes with the lowest incoming edges.
///
/// Such linker can be created by passing `min_incoming` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct MinIncomingLinker;

impl NamedParam<BoxedLinker> for MinIncomingLinker {
    fn name(&self) -> &'static str {
        "min_incoming"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes of the first graph with the lowest count of incoming edges to the nodes of the second graph with the same property."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, false)
    }
}

impl Linker for MinIncomingLinker {}

/// A bidirectional linker that connects graph by targeting their nodes with the lowest incoming edges.
///
/// Such linker can be created by passing `min_incoming_bi` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
pub struct BidirectionalSourcesLinker;

impl NamedParam<BoxedLinker> for BidirectionalSourcesLinker {
    fn name(&self) -> &'static str {
        "min_incoming_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes of the first graph with the lowest count of incoming edges to the nodes of the second graph with the same property, and vice-versa."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, true)
    }
}

impl Linker for BidirectionalSourcesLinker {}

fn try_with_params(params: &str, bidirectional: bool) -> Result<BoxedLinker> {
    let context = "while building a sources linker";
    if !params.is_empty() {
        return Err(anyhow!("unexpected parameter(s)")).context(context);
    }
    Ok(Box::new(move |g1, g2| {
        let min_incoming_1 = min_incoming(g1);
        let min_incoming_2 = min_incoming(g2);
        let capacity = if bidirectional {
            (min_incoming_1.len() * min_incoming_2.len()) << 1
        } else {
            min_incoming_1.len() * min_incoming_2.len()
        };
        let mut links = Vec::with_capacity(capacity);
        min_incoming_1.iter().for_each(|n1| {
            min_incoming_2.iter().for_each(|n2| {
                links.push(InterGraphEdge::FirstToSecond(*n1, *n2));
                if bidirectional {
                    links.push(InterGraphEdge::SecondToFirst(*n2, *n1));
                }
            });
        });
        links
    }))
}

fn min_incoming(g: &Graph) -> Vec<usize> {
    let mut n_incoming = vec![0; g.n_nodes()];
    g.iter_edges().for_each(|(_, i)| {
        n_incoming[i] += 1;
    });
    let min_n_incoming = n_incoming.iter().min().copied().unwrap_or_default();
    n_incoming
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| if n == min_n_incoming { Some(i) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_incoming_too_much_params() {
        assert!(MinIncomingLinker.try_with_params("1").is_err());
    }

    #[test]
    fn test_min_incoming_ok() {
        let mut g0 = Graph::default();
        g0.new_node();
        g0.new_node();
        let mut g1 = Graph::default();
        g1.new_edge(0, 1);
        let linker = MinIncomingLinker.try_with_params("").unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::FirstToSecond(1, 0)
            ],
            linker(&g0, &g1)
        );
    }

    #[test]
    fn test_min_incoming_bi_too_much_params() {
        assert!(BidirectionalSourcesLinker.try_with_params("1").is_err());
    }

    #[test]
    fn test_min_incoming_bi_ok() {
        let mut g0 = Graph::default();
        g0.new_node();
        g0.new_node();
        let mut g1 = Graph::default();
        g1.new_edge(0, 1);
        let linker = BidirectionalSourcesLinker.try_with_params("").unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0),
                InterGraphEdge::FirstToSecond(1, 0),
                InterGraphEdge::SecondToFirst(0, 1),
            ],
            linker(&g0, &g1)
        );
    }
}
