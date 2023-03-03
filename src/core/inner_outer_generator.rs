use super::InnerGraph;
use crate::{Graph, InterGraphEdge, NodeIndexType};
use petgraph::EdgeType;
use rand::{distributions::Standard, Rng, SeedableRng};
use rayon::prelude::*;

/// A structure dedicated to the generation of Inner/outer graphs.
#[derive(Default)]
pub struct InnerOuterGenerator {
    generation_step_listeners: Vec<BoxedGenerationStepListener>,
}

/// Key steps of the Inner/outer graph generation process.
pub enum InnerOuterGenerationStep {
    /// The outer graph generation has just begun
    OuterGeneration,
    /// The inner graphs generation has just begun
    InnerGeneration,
    /// The linking has just begun
    Linking,
}

pub type BoxedGenerationStepListener = Box<dyn Fn(InnerOuterGenerationStep)>;

impl InnerOuterGenerator {
    /// Builds an inner/outer graph using two graph generators and a linker.
    ///
    /// First, the outer graph generator is used to built the outer graph.
    /// Then, for each node in the outer graph, an inner graph is created using the dedicated generator.
    /// Finally, for each edge in the outer graph, the two corresponding inner graphs are joined with the linker.
    ///
    /// ```
    /// # use crusti_g2io::{Graph, ChainGeneratorFactory, InnerOuterGenerator, InterGraphEdge, NodeIndexType, FirstToFirstLinker, NamedParam, linkers::BoxedLinker, ParameterValue};
    /// use petgraph::Directed;
    /// use rand::SeedableRng;
    /// use rand_pcg::Pcg32;
    ///
    /// let first_node_edge_selector: BoxedLinker<Directed, Pcg32> = FirstToFirstLinker::default().try_with_params(vec![]).unwrap();
    /// let inner_outer_generator = InnerOuterGenerator::default();
    /// let inner_outer = inner_outer_generator.new_inner_outer(
    ///     ChainGeneratorFactory::default().try_with_params(vec![ParameterValue::PositiveInteger(2)]).unwrap(),
    ///     ChainGeneratorFactory::default().try_with_params(vec![ParameterValue::PositiveInteger(3)]).unwrap(),
    ///     first_node_edge_selector,
    ///     &mut Pcg32::from_entropy(),
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
    pub fn new_inner_outer<F, G, H, R, Ty>(
        &self,
        outer_graph_builder: F,
        inner_graph_builder: G,
        linker: H,
        rng: &mut R,
    ) -> Graph<Ty>
    where
        F: Fn(&mut R) -> Graph<Ty>,
        G: Fn(&mut R) -> Graph<Ty> + Sync + Send,
        H: Fn(InnerGraph<Ty>, InnerGraph<Ty>, &mut R) -> Vec<InterGraphEdge> + Sync,
        R: Rng + SeedableRng + Send,
        Ty: EdgeType + Send + Sync,
    {
        let outer_graph = self.generate_outer_graph(outer_graph_builder, rng);
        let inner_graphs = self.generate_inner_graphs(&outer_graph, inner_graph_builder, rng);
        let mut global_graph = Graph::<Ty>::default();
        inner_graphs
            .iter()
            .for_each(|g| global_graph.append_graph(g));
        self.link(&outer_graph, &inner_graphs, linker, rng)
    }

    fn generate_outer_graph<F, R, Ty>(&self, outer_graph_builder: F, rng: &mut R) -> Graph<Ty>
    where
        F: Fn(&mut R) -> Graph<Ty>,
        R: Rng + SeedableRng + Send,
        Ty: EdgeType,
    {
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::OuterGeneration));
        (outer_graph_builder)(rng)
    }

    fn generate_inner_graphs<G, R, Ty>(
        &self,
        outer_graph: &Graph<Ty>,
        inner_graph_builder: G,
        rng: &mut R,
    ) -> Vec<Graph<Ty>>
    where
        G: Fn(&mut R) -> Graph<Ty> + Sync + Send,
        R: Rng + SeedableRng + Send,
        Ty: EdgeType + Send,
    {
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::InnerGeneration));
        let inner_seeds: Vec<u64> = rng
            .sample_iter(Standard)
            .take(outer_graph.n_nodes())
            .collect();
        inner_seeds
            .into_par_iter()
            .map(|s| R::seed_from_u64(s))
            .map(|mut r| inner_graph_builder(&mut r))
            .collect()
    }

    fn link<H, R, Ty>(
        &self,
        outer_graph: &Graph<Ty>,
        inner_graphs: &[Graph<Ty>],
        linker: H,
        rng: &mut R,
    ) -> Graph<Ty>
    where
        H: Fn(InnerGraph<Ty>, InnerGraph<Ty>, &mut R) -> Vec<InterGraphEdge> + Sync,
        R: Rng + SeedableRng + Send,
        Ty: EdgeType + Send + Sync,
    {
        let mut global_graph = Graph::default();
        inner_graphs
            .iter()
            .for_each(|g| global_graph.append_graph(g));
        let cumulated_n_nodes =
            inner_graphs
                .iter()
                .fold(Vec::with_capacity(1 + outer_graph.n_nodes()), |mut v, g| {
                    if v.is_empty() {
                        v.append(&mut vec![0, g.n_nodes()]);
                    } else {
                        v.push(v.last().unwrap() + g.n_nodes());
                    };
                    v
                });
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::Linking));
        self.add_linking_edges(
            outer_graph,
            inner_graphs,
            &cumulated_n_nodes,
            global_graph,
            linker,
            rng,
        )
    }

    fn add_linking_edges<H, R, Ty>(
        &self,
        outer_graph: &Graph<Ty>,
        inner_graphs: &[Graph<Ty>],
        cumulated_n_nodes: &[usize],
        mut global_graph: Graph<Ty>,
        linker: H,
        rng: &mut R,
    ) -> Graph<Ty>
    where
        H: Fn(InnerGraph<Ty>, InnerGraph<Ty>, &mut R) -> Vec<InterGraphEdge> + Sync,
        R: Rng + SeedableRng + Send,
        Ty: EdgeType + Send + Sync,
    {
        let raw_edges = outer_graph.petgraph().raw_edges();
        let seeds: Vec<u64> = rng.sample_iter(Standard).take(raw_edges.len()).collect();
        let all_global_edges = (0..raw_edges.len())
            .into_par_iter()
            .map(|i| {
                let petgraph_edge = &raw_edges[i];
                let mut rng = R::seed_from_u64(seeds[i]);
                let outer_edge = (
                    petgraph_edge.source().index(),
                    petgraph_edge.target().index(),
                );
                let inter_attacks = (linker)(
                    (outer_edge.0, &inner_graphs[outer_edge.0]).into(),
                    (outer_edge.1, &inner_graphs[outer_edge.1]).into(),
                    &mut rng,
                );
                inter_attacks
                    .iter()
                    .map(|inter_edge| {
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
                        (global_node_ids.0, global_node_ids.1)
                    })
                    .collect::<Vec<(NodeIndexType, NodeIndexType)>>()
            })
            .collect::<Vec<Vec<(NodeIndexType, NodeIndexType)>>>();
        all_global_edges
            .into_iter()
            .flatten()
            .for_each(|(from, to)| global_graph.new_edge(from, to));
        global_graph
    }

    /// Adds a listener to this generator to track the generation process.
    ///
    /// At key points of the generation process, the provided listener will be advised a new step has come.
    pub fn add_generation_step_listener(&mut self, l: BoxedGenerationStepListener) {
        self.generation_step_listeners.push(l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeIndexType;
    use petgraph::Directed;
    use rand_pcg::Pcg32;

    #[test]
    fn test_inner_outer() {
        let circle_builder = |_: &mut Pcg32| {
            const N: usize = 3;
            let mut g = Graph::with_capacity(N, N);
            for i in 0..N - 1 {
                g.new_edge(i, i + 1);
            }
            g.new_edge(N - 1, 0);
            g
        };
        let chain_builder = |_: &mut Pcg32| {
            let mut g = Graph::with_capacity(2, 1);
            g.new_edge(0, 1);
            g
        };
        let first_node_edge_selector =
            |_: InnerGraph<Directed>, _: InnerGraph<Directed>, _: &mut Pcg32| {
                vec![InterGraphEdge::FirstToSecond(0, 0)]
            };
        let inner_outer_generator = InnerOuterGenerator::default();
        let inner_outer = inner_outer_generator.new_inner_outer(
            chain_builder,
            circle_builder,
            first_node_edge_selector,
            &mut Pcg32::seed_from_u64(0),
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
        let circle_builder = |_: &mut Pcg32| {
            const N: usize = 3;
            let mut g = Graph::with_capacity(N, N);
            for i in 0..N - 1 {
                g.new_edge(i, i + 1);
            }
            g.new_edge(N - 1, 0);
            g
        };
        let chain_builder = |_: &mut Pcg32| {
            let mut g = Graph::with_capacity(2, 1);
            g.new_edge(0, 1);
            g
        };
        let first_node_edge_selector =
            |_: InnerGraph<Directed>, _: InnerGraph<Directed>, _: &mut Pcg32| {
                vec![InterGraphEdge::SecondToFirst(0, 0)]
            };
        let inner_outer_generator = InnerOuterGenerator::default();
        let inner_outer = inner_outer_generator.new_inner_outer(
            chain_builder,
            circle_builder,
            first_node_edge_selector,
            &mut Pcg32::seed_from_u64(0),
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
