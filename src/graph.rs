use petgraph::graph::NodeIndex;
use petgraph::Directed;
use rand::Rng;

pub type NodeIndexType = usize;

#[derive(Default)]
pub struct Graph(petgraph::Graph<(), (), Directed, NodeIndexType>);

impl Graph {
    pub fn with_capacity(n_nodes: usize, n_edges: usize) -> Self {
        Self(petgraph::Graph::with_capacity(n_nodes, n_edges))
    }

    pub fn new_inner_outer<F, G, H, R>(
        outer_graph_builder: F,
        inner_graph_builder: G,
        inter_graph_edges_selector: H,
        rng: &mut R,
    ) -> Self
    where
        F: Fn(&mut R) -> Graph,
        G: Fn(&mut R) -> Graph,
        H: Fn(&Graph, &Graph) -> Vec<(NodeIndexType, NodeIndexType)>,
        R: Rng,
    {
        let outer = (outer_graph_builder)(rng);
        let inner_graphs = (0..outer.n_nodes())
            .map(|_| (inner_graph_builder)(rng))
            .collect::<Vec<Graph>>();
        let mut global_graph = Graph::default();
        inner_graphs
            .iter()
            .for_each(|g| global_graph.append_graph(g));
        let cumulated_n_nodes =
            inner_graphs
                .iter()
                .fold(Vec::with_capacity(1 + outer.n_nodes()), |mut v, g| {
                    if v.is_empty() {
                        v.append(&mut vec![0, g.n_nodes()]);
                    } else {
                        v.push(v.last().unwrap() + g.n_nodes());
                    };
                    v
                });
        outer.iter_edges().for_each(|outer_edge| {
            let inter_attacks = (inter_graph_edges_selector)(
                &inner_graphs[outer_edge.0],
                &inner_graphs[outer_edge.1],
            );
            inter_attacks.iter().for_each(|inter_edge| {
                global_graph.new_edge(
                    inter_edge.0 + cumulated_n_nodes[outer_edge.0],
                    inter_edge.1 + cumulated_n_nodes[outer_edge.1],
                )
            });
        });
        global_graph
    }

    pub fn new_node(&mut self) {
        self.0.add_node(());
    }

    pub fn n_nodes(&self) -> usize {
        self.0.node_count()
    }

    pub fn new_edge(&mut self, from: NodeIndexType, to: NodeIndexType) {
        (self.n_nodes()..=from).for_each(|_| {
            self.0.add_node(());
        });
        (self.n_nodes()..=to).for_each(|_| {
            self.0.add_node(());
        });
        self.0
            .update_edge(NodeIndex::from(from), NodeIndex::from(to), ());
    }

    pub fn n_edges(&self) -> usize {
        self.0.edge_count()
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (NodeIndexType, NodeIndexType)> + '_ {
        self.0
            .raw_edges()
            .iter()
            .map(|e| (e.source().index(), e.target().index()))
    }

    fn append_graph(&mut self, g: &Graph) {
        let self_n_nodes = self.n_nodes();
        let g_n_nodes = g.n_nodes();
        self.0.reserve_nodes(g_n_nodes);
        (0..g_n_nodes).for_each(|_| {
            self.0.add_node(());
        });
        self.0.reserve_edges(g.n_edges());
        for edge in g.0.raw_edges() {
            self.new_edge(
                edge.source().index() + self_n_nodes,
                edge.target().index() + self_n_nodes,
            );
        }
    }

    pub(crate) fn petgraph(&self) -> &petgraph::Graph<(), (), Directed, NodeIndexType> {
        &self.0
    }
}

impl From<petgraph::Graph<(), (), Directed, NodeIndexType>> for Graph {
    fn from(g: petgraph::Graph<(), (), Directed, NodeIndexType>) -> Self {
        Self(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::ThreadRng;

    #[test]
    pub fn test_new_edge_adds_node() {
        let mut g = Graph::default();
        assert_eq!(0, g.n_nodes());
        assert_eq!(0, g.n_edges());
        g.new_edge(0, 1);
        assert_eq!(2, g.n_nodes());
        assert_eq!(1, g.n_edges());
        g.new_edge(0, 0);
        assert_eq!(2, g.n_nodes());
        assert_eq!(2, g.n_edges());
    }

    #[test]
    fn test_iter_edges() {
        let g = Graph(petgraph::Graph::from_edges(&[(0, 1), (0, 0)]));
        assert_eq!(
            vec![(0, 1), (0, 0)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        )
    }

    #[test]
    fn test_append_graph() {
        let mut g0 = Graph(petgraph::Graph::from_edges(&[(0, 1)]));
        assert_eq!(2, g0.n_nodes());
        assert_eq!(
            vec![(0, 1)],
            g0.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
        let g1 = Graph(petgraph::Graph::from_edges(&[(1, 0)]));
        g0.append_graph(&g1);
        assert_eq!(4, g0.n_nodes());
        assert_eq!(
            vec![(0, 1), (3, 2)],
            g0.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
        let g2 = Graph(petgraph::Graph::from_edges(&[(0, 1), (1, 0)]));
        g0.append_graph(&g2);
        assert_eq!(6, g0.n_nodes());
        assert_eq!(
            vec![(0, 1), (3, 2), (4, 5), (5, 4)],
            g0.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        );
    }

    #[test]
    fn test_inner_outer() {
        let circle_builder = |_: &mut ThreadRng| {
            const N: usize = 3;
            let mut g = Graph::with_capacity(N, N);
            for i in 0..N - 1 {
                g.new_edge(i, i + 1);
            }
            g.new_edge(N - 1, 0);
            g
        };
        let chain_builder = |_: &mut ThreadRng| {
            let mut g = Graph::with_capacity(2, 1);
            g.new_edge(0, 1);
            g
        };
        let first_node_edge_selector = |_: &Graph, _: &Graph| vec![(0, 0)];
        let inner_outer = Graph::new_inner_outer(
            chain_builder,
            circle_builder,
            first_node_edge_selector,
            &mut rand::thread_rng(),
        );
        let mut expected = inner_outer
            .iter_edges()
            .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
        expected.sort_unstable();
        assert_eq!(
            vec![(0, 1), (0, 3), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)],
            expected
        );
    }
}
