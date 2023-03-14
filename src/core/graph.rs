use petgraph::graph::NodeIndex;
use petgraph::EdgeType;

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
/// use petgraph::Directed;
///
/// fn path(n: usize) -> Graph<Directed> {
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
/// # path(2);
/// ```
pub struct Graph<Ty>(petgraph::Graph<(), (), Ty, NodeIndexType>)
where
    Ty: EdgeType;

impl<Ty> Default for Graph<Ty>
where
    Ty: EdgeType,
{
    fn default() -> Self {
        Self(petgraph::Graph::<(), (), Ty, NodeIndexType>::default())
    }
}

/// An edge between the nodes of two different graphs.
///
/// The edge is defined by its direction (from the first graph to the second, or from the second to the first)
/// and the labels of the nodes involved in the edge.
#[derive(Debug, PartialEq, Eq)]
pub enum InterGraphEdge {
    /// An edge from the first graph to the second
    FirstToSecond(NodeIndexType, NodeIndexType),
    /// An edge from the second graph to the first
    SecondToFirst(NodeIndexType, NodeIndexType),
}

impl<Ty> Graph<Ty>
where
    Ty: EdgeType,
{
    /// Builds a new graph with an initial capacity for nodes and edges.
    ///
    /// These capacity are only size hints; they can improve performance but a graph built by this method can handle any number of nodes and edges.
    pub fn with_capacity(n_nodes: usize, n_edges: usize) -> Self {
        Self(petgraph::Graph::with_capacity(n_nodes, n_edges))
    }

    /// Adds a new node to the graph, using the lowest free positive integer label.
    ///
    /// ```
    /// # use crusti_g2io::Graph;
    /// use petgraph::Directed;
    ///
    /// let mut graph = Graph::<Directed>::default();
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
    /// If one of the nodes involved in the edge is not defined yet, this functions adds it the the graph
    /// and defines all the missing nodes which have a label between 0 and the label of the new node.
    ///
    /// Beware: functions allows the addition of the same edge multiple times.
    ///
    /// ```
    /// # use crusti_g2io::Graph;
    /// use petgraph::Directed;
    ///
    /// let mut graph = Graph::<Directed>::default();
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
            .add_edge(NodeIndex::from(from), NodeIndex::from(to), ());
    }

    /// Returns the number of edges contained in the graph.
    pub fn n_edges(&self) -> usize {
        self.0.edge_count()
    }

    /// Returns an iterator to the edges of this graph.
    ///
    /// Edges are given as couples of labels.
    ///
    /// This functions returns a view to the edges just the way they have been added.
    /// In particular, if an edge was added `n` times, the iterator will yield it `n` times.
    /// ```
    /// # use crusti_g2io::Graph;
    /// use petgraph::EdgeType;
    ///
    /// fn debug_graph<Ty>(g: &Graph<Ty>) where Ty: EdgeType {
    ///     println!("g has {} nodes", g.n_nodes());
    ///     g.iter_edges().for_each(|(s,t)| println!("there is an edge from {} to {}", s, t));
    /// }
    /// # debug_graph(&Graph::<petgraph::Directed>::default());
    /// ```
    pub fn iter_edges(&self) -> impl Iterator<Item = (NodeIndexType, NodeIndexType)> + '_ {
        self.0
            .raw_edges()
            .iter()
            .map(|e| (e.source().index(), e.target().index()))
    }

    /// Removes the edge given the two nodes it links (source may be given first in case the graph is directed).
    ///
    /// # Panics
    ///
    /// If the provided nodes do not match any edge, this function panics.
    pub fn remove_edge(&mut self, from: NodeIndexType, to: NodeIndexType) {
        let index = self
            .0
            .find_edge(from.into(), to.into())
            .unwrap_or_else(|| panic!("no such edge (from {} to {})", from, to));
        self.0.remove_edge(index).unwrap();
    }

    pub(crate) fn append_graph(&mut self, g: &Graph<Ty>) {
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

    pub(crate) fn petgraph(&self) -> &petgraph::Graph<(), (), Ty, NodeIndexType> {
        &self.0
    }
}

/// A structure used to store an inner graph.
///
/// Its main purpose is to associate an index to a graph, allowing linkers to cache data.
pub struct InnerGraph<'a, Ty>
where
    Ty: EdgeType,
{
    index: usize,
    graph: &'a Graph<Ty>,
}

impl<Ty> InnerGraph<'_, Ty>
where
    Ty: EdgeType,
{
    /// Returns the index of the inner graph.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the inner graph.
    pub fn graph(&self) -> &Graph<Ty> {
        self.graph
    }
}

impl<'a, Ty> From<(usize, &'a Graph<Ty>)> for InnerGraph<'a, Ty>
where
    Ty: EdgeType,
{
    fn from(t: (usize, &'a Graph<Ty>)) -> Self {
        Self {
            index: t.0,
            graph: t.1,
        }
    }
}

impl<Ty> From<petgraph::Graph<(), (), Ty, NodeIndexType>> for Graph<Ty>
where
    Ty: EdgeType,
{
    fn from(g: petgraph::Graph<(), (), Ty, NodeIndexType>) -> Self {
        Self(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Directed;

    #[test]
    pub fn test_new_edge_adds_node() {
        let mut g: Graph<Directed> = Graph::default();
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
        let g: Graph<Directed> = Graph(petgraph::Graph::from_edges(&[(0, 1), (0, 0)]));
        assert_eq!(
            vec![(0, 1), (0, 0)],
            g.iter_edges()
                .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
        )
    }

    #[test]
    fn test_append_graph() {
        let mut g0: Graph<Directed> = Graph(petgraph::Graph::from_edges(&[(0, 1)]));
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
}
