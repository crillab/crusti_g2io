use crate::graph::Graph;
use anyhow::{anyhow, Result};
use petgraph_gen::barabasi_albert_graph;
use rand::Rng;

pub fn new_chain(n: usize) -> Graph {
    match n {
        0 => Graph::default(),
        1 => {
            let mut g = Graph::with_capacity(1, 0);
            g.new_node();
            g
        }
        _ => {
            let mut g = Graph::with_capacity(n, n - 1);
            (0..n - 1).for_each(|i| g.new_edge(i, i + 1));
            g
        }
    }
}

pub fn new_barabasi_albert<R>(n: usize, m: usize, rng: &mut R) -> Graph
where
    R: Rng,
{
    barabasi_albert_graph(rng, n, m, None).into()
}

pub type BoxedGenerator<R> = Box<dyn Fn(&mut R) -> Graph>;

pub fn from_str<R>(s: &str) -> Result<BoxedGenerator<R>>
where
    R: Rng,
{
    let make_err = || anyhow!(r#"cannot build a generator from the string "{}""#, s);
    let (kind, str_params) = s.split_once('/').ok_or_else(make_err)?;
    let params = str_params
        .split(',')
        .map(|i| str::parse::<i32>(i).map_err(|_| make_err()))
        .collect::<Result<Vec<i32>>>()?;
    let r: BoxedGenerator<R> = match (kind, params.len()) {
        ("chain", 1) => Box::new(move |_: &mut R| new_chain(params[0] as usize)),
        ("ba", 2) => Box::new(move |rng: &mut R| {
            new_barabasi_albert(params[0] as usize, params[1] as usize, rng)
        }),
        _ => return Err(make_err()),
    };
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeIndexType;

    #[test]
    fn test_chain_of_zero() {
        let g = new_chain(0);
        assert_eq!(0, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_chain_of_one() {
        let g = new_chain(1);
        assert_eq!(1, g.n_nodes());
        assert_eq!(0, g.n_edges());
    }

    #[test]
    fn test_chain() {
        let g = new_chain(3);
        assert_eq!(3, g.n_nodes());
        assert_eq!(
            vec![(0, 1), (1, 2)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }

    #[test]
    fn test_barabasi_star() {
        let g = new_barabasi_albert(4, 3, &mut rand::thread_rng());
        assert_eq!(
            vec![(0, 1), (0, 2), (0, 3)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }
}
