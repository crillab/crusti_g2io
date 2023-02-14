use super::{BoxedLinker, Linker};
use crate::{core::InnerGraph, InterGraphEdge, NamedParam};
use anyhow::{anyhow, Context, Result};
use std::sync::{Arc, Mutex};

pub type Cache = Arc<Mutex<Vec<Option<Vec<usize>>>>>;

/// A linker that connects graph by targeting their nodes with the lowest incoming edges.
///
/// Such linker can be created by passing `min_incoming` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct MinIncomingLinker {
    min_incoming_cache: Cache,
}

impl NamedParam<BoxedLinker> for MinIncomingLinker {
    fn name(&self) -> &'static str {
        "min_incoming"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes of the first graph with the lowest count of incoming edges to the nodes of the second graph with the same property."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, Arc::clone(&self.min_incoming_cache), false)
    }
}

impl Linker for MinIncomingLinker {}

/// A bidirectional linker that connects graph by targeting their nodes with the lowest incoming edges.
///
/// Such linker can be created by passing `min_incoming_bi` to [`linkers::linker_from_str`](crate::linkers#linker_from_str).
#[derive(Default)]
pub struct BidirectionalMinIncomingLinker {
    min_incoming_cache: Cache,
}

impl NamedParam<BoxedLinker> for BidirectionalMinIncomingLinker {
    fn name(&self) -> &'static str {
        "min_incoming_bi"
    }

    fn description(&self) -> Vec<&'static str> {
        vec!["Links the nodes of the first graph with the lowest count of incoming edges to the nodes of the second graph with the same property, and vice-versa."]
    }

    fn try_with_params(&self, params: &str) -> Result<BoxedLinker> {
        try_with_params(params, Arc::clone(&self.min_incoming_cache), true)
    }
}

impl Linker for BidirectionalMinIncomingLinker {}

fn try_with_params(
    params: &str,
    min_incoming_cache: Cache,
    bidirectional: bool,
) -> Result<BoxedLinker> {
    let context = "while building a sources linker";
    if !params.is_empty() {
        return Err(anyhow!("unexpected parameter(s)")).context(context);
    }
    Ok(Box::new(move |g1, g2| {
        let min_incoming_1 = compute_min_incoming(&g1, Arc::clone(&min_incoming_cache));
        let min_incoming_2 = compute_min_incoming(&g2, Arc::clone(&min_incoming_cache));
        let mut links = Vec::new();
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

fn compute_min_incoming(g: &InnerGraph, min_incoming_cache: Cache) -> Vec<usize> {
    let mut cache_handle = min_incoming_cache.lock().unwrap();
    if cache_handle.len() <= g.index() {
        cache_handle.resize(1 + g.index(), None);
    } else if let Some(v) = &cache_handle[g.index()] {
        return v.clone();
    }
    std::mem::drop(cache_handle);
    let mut n_incoming = vec![0; g.graph().n_nodes()];
    g.graph().iter_edges().for_each(|(_, i)| {
        n_incoming[i] += 1;
    });
    let min_n_incoming = n_incoming.iter().min().copied().unwrap_or_default();
    let v = n_incoming
        .into_iter()
        .enumerate()
        .filter_map(|(i, n)| if n == min_n_incoming { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let some_v_clone = Some(v.clone());
    min_incoming_cache.lock().unwrap()[g.index()] = some_v_clone;
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[test]
    fn test_min_incoming_too_much_params() {
        assert!(MinIncomingLinker::default().try_with_params("1").is_err());
    }

    #[test]
    fn test_min_incoming_ok() {
        let mut g0 = Graph::default();
        g0.new_node();
        g0.new_node();
        let mut g1 = Graph::default();
        g1.new_edge(0, 1);
        let linker = MinIncomingLinker::default().try_with_params("").unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::FirstToSecond(1, 0)
            ],
            linker((0, &g0).into(), (1, &g1).into())
        );
    }

    #[test]
    fn test_min_incoming_bi_too_much_params() {
        assert!(BidirectionalMinIncomingLinker::default()
            .try_with_params("1")
            .is_err());
    }

    #[test]
    fn test_min_incoming_bi_ok() {
        let mut g0 = Graph::default();
        g0.new_node();
        g0.new_node();
        let mut g1 = Graph::default();
        g1.new_edge(0, 1);
        let linker = BidirectionalMinIncomingLinker::default()
            .try_with_params("")
            .unwrap();
        assert_eq!(
            vec![
                InterGraphEdge::FirstToSecond(0, 0),
                InterGraphEdge::SecondToFirst(0, 0),
                InterGraphEdge::FirstToSecond(1, 0),
                InterGraphEdge::SecondToFirst(0, 1),
            ],
            linker((0, &g0).into(), (1, &g1).into())
        );
    }
}
