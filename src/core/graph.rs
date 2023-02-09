use petgraph::graph::NodeIndex;
use petgraph::Directed;
use rand::Rng;

/// The node label type
pub type NodeIndexType = usize;

/// A directed graph, where nodes are labeled by integers and edges have no labels.
///
/// Graphs can be created empty using the [`default`](Default::default) or with the [`with_capacity`](Self#with_capacity) function.
///
/// Nodes are labelled by positive integer values.
/// New nodes can be created by the [`new_node`](Self#new_node) function (which adds the node with the lowest integer label that is not already present)
/// or when added an edge referring to it. In the latter case, all the nodes with labels between 0 and the highest label are created.
///
/// ```
/// # use crusti_g2io::Graph;
/// fn chain(n: usize) -> Graph {
///     let graph = match n {
///         0 => Graph::default(),
///         1 => {
///             let mut g = Graph::with_capacity(1, 0);
///             g.new_node();
///             g
///         }
///         _ => {
///             let mut g = Graph::with_capacity(n, n-1);
///             (0..n-1).for_each(|i| g.new_edge(i.into(), (i+1).into()));
///             g
///         }
///     };
///     assert_eq!(n, graph.n_nodes());
///     assert_eq!(n-1, graph.n_edges());
///     graph
/// }
/// # chain(2);
/// ```
#[derive(Default)]
pub struct Graph(petgraph::Graph<(), (), Directed, NodeIndexType>);

/// An edge between the nodes of two different graphs.
///
/// The edge if defined by its direction (from the first graph to the second, or from the second to the first)
/// and the labels of the nodes involved in the edge.
#[derive(Debug, PartialEq, Eq)]
pub enum InterGraphEdge {
    /// An edge from the first graph to the second
    FirstToSecond(NodeIndexType, NodeIndexType),
    /// An edge from the second graph to the first
    SecondToFirst(NodeIndexType, NodeIndexType),
}

impl Graph {
    /// Builds a new graph with an initial capacity for nodes and edges.
    ///
    /// These capacity are only size hints ; they can improve performance but a graph built by this method can handle any number of nodes and edges.
    pub fn with_capacity(n_nodes: usize, n_edges: usize) -> Self {
        Self(petgraph::Graph::with_capacity(n_nodes, n_edges))
    }

    /// Builds an inner/outer graph using two graph generators and a linker.
    ///
    /// First, the outer graph generator is used to built the outer graph.
    /// Then, for each node in the outer graph, an inner graph is created using the dedicated generator.
    /// Finally, for each edge in the outer graph, the two corresponding inner graphs are joined with the linker.
    ///
    /// ```
    /// # use crusti_g2io::{Graph, ChainGeneratorFactory, InterGraphEdge, NodeIndexType, FirstToFirstLinker, NamedParam};
    /// let first_node_edge_selector = FirstToFirstLinker::default().try_with_params("").unwrap();
    /// let inner_outer = Graph::new_inner_outer(
    ///     ChainGeneratorFactory::default().try_with_params("2").unwrap(),
    ///     ChainGeneratorFactory::default().try_with_params("3").unwrap(),
    ///     first_node_edge_selector,
    ///     &mut rand::thread_rng(),
    /// );
    /// let mut expected = inner_outer
    ///     .iter_edges()
    ///     .collect::<Vec<(NodeIndexType, NodeIndexType)>>();
    /// expected.sort_unstable();
    /// assert_eq!(
    ///     vec![(0, 1), (0, 3), (1, 2), (3, 4), (4, 5)],
    ///     expected
    /// );
    /// ```
    pub fn new_inner_outer<F, G, H, R>(
        outer_graph_builder: F,
        inner_graph_builder: G,
        linker: H,
        rng: &mut R,
    ) -> Self
    where
        F: Fn(&mut R) -> Graph,
        G: Fn(&mut R) -> Graph,
        H: Fn(InnerGraph, InnerGraph) -> Vec<InterGraphEdge>,
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
            let inter_attacks = (linker)(
                (outer_edge.0, &inner_graphs[outer_edge.0]).into(),
                (outer_edge.1, &inner_graphs[outer_edge.1]).into(),
            );
            inter_attacks.iter().for_each(|inter_edge| {
                let global_node_ids = match inter_edge {
                    InterGraphEdge::FirstToSecond(a, b) => (
                        a + cumulated_n_nodes[outer_edge.0],
                        b + cumulated_n_nodes[outer_edge.1],
                    ),
                    InterGraphEdge::SecondToFirst(b, a) => (
                        a + cumulated_n_nodes[outer_edge.1],
                        b + cumulated_n_nodes[outer_edge.0],
                    ),
                };
                global_graph.new_edge(global_node_ids.0, global_node_ids.1);
            });
        });
        global_graph
    }

    /// Adds a new node to the graph, using the lowest free positive integer label.
    ///
    /// ```
    /// # use crusti_g2io::Graph;
    /// let mut graph = Graph::default();
    /// assert_eq!(0, graph.n_nodes());
    /// graph.new_node();
    /// assert_eq!(1, graph.n_nodes());
    /// graph.new_edge(0_usize.into(), 0_usize.into());
    /// assert_eq!(1, graph.n_nodes());
    /// ```
    pub fn new_node(&mut self) {
        self.0.add_node(());
    }

    /// Returns the number of nodes contained in the graph.
    pub fn n_nodes(&self) -> usize {
        self.0.node_count()
    }

    /// Adds an edge to the graph.
    ///
    /// If one of the node involved in the edge is not defined yet, this functions adds it the the graph
    /// and defines all the missing nodes which have a label between 0 and the label of the new node.
    ///
    /// ```
    /// # use crusti_g2io::Graph;
    /// let mut graph = Graph::default();
    /// assert_eq!(0, graph.n_nodes());
    /// graph.new_edge(2_usize.into(), 3_usize.into());
    /// assert_eq!(4, graph.n_nodes());
    /// ```
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

    /// Returns the number of edges contained in the graph.
    pub fn n_edges(&self) -> usize {
        self.0.edge_count()
    }

    /// Returns an iterator to the edges of this node.
    ///
    /// Edges are given as couples of labels.
    ///
    /// ```
    /// # use crusti_g2io::Graph;
    /// fn debug_graph(g: &Graph) {
    ///     println!("g has {} nodes", g.n_nodes());
    ///     g.iter_edges().for_each(|(s,t)| println!("there is an edge from {} to {}", s, t));
    /// }
    /// # debug_graph(&Graph::default());
    /// ```
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

/// A structure used to store an inner graph.
///
/// Its main purpose is to associate an index to a graph, allowing linkers to cache data.
pub struct InnerGraph<'a> {
    index: usize,
    graph: &'a Graph,
}

impl InnerGraph<'_> {
    /// Returns the index of the inner graph.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the inner graph.
    pub fn graph(&self) -> &Graph {
        self.graph
    }
}

impl<'a> From<(usize, &'a Graph)> for InnerGraph<'a> {
    fn from(t: (usize, &'a Graph)) -> Self {
        Self {
            index: t.0,
            graph: t.1,
        }
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
        let first_node_edge_selector =
            |_: InnerGraph, _: InnerGraph| vec![InterGraphEdge::FirstToSecond(0, 0)];
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

    #[test]
    fn test_inner_inv_outer() {
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
        let first_node_edge_selector =
            |_: InnerGraph, _: InnerGraph| vec![InterGraphEdge::SecondToFirst(0, 0)];
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
            vec![(0, 1), (1, 2), (2, 0), (3, 0), (3, 4), (4, 5), (5, 3)],
            expected
        );
    }
}
