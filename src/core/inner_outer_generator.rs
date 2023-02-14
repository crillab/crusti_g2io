use super::InnerGraph;
use crate::{Graph, InterGraphEdge};
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
    /// # use crusti_g2io::{Graph, ChainGeneratorFactory, InnerOuterGenerator, InterGraphEdge, NodeIndexType, FirstToFirstLinker, NamedParam};
    /// use rand::SeedableRng;
    /// use rand_pcg::Pcg32;
    ///
    /// let first_node_edge_selector = FirstToFirstLinker::default().try_with_params("").unwrap();
    /// let inner_outer_generator = InnerOuterGenerator::default();
    /// let inner_outer = inner_outer_generator.new_inner_outer(
    ///     ChainGeneratorFactory::default().try_with_params("2").unwrap(),
    ///     ChainGeneratorFactory::default().try_with_params("3").unwrap(),
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
    pub fn new_inner_outer<F, G, H, R>(
        &self,
        outer_graph_builder: F,
        inner_graph_builder: G,
        linker: H,
        rng: &mut R,
    ) -> Graph
    where
        F: Fn(&mut R) -> Graph,
        G: Fn(&mut R) -> Graph + Sync + Send,
        H: Fn(InnerGraph, InnerGraph) -> Vec<InterGraphEdge>,
        R: Rng + SeedableRng + Send,
    {
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::OuterGeneration));
        let outer = (outer_graph_builder)(rng);
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::InnerGeneration));
        let inner_seeds: Vec<u64> = rng.sample_iter(Standard).take(outer.n_nodes()).collect();
        let inner_graphs = inner_seeds
            .into_par_iter()
            .map(|s| R::seed_from_u64(s))
            .map(|mut r| inner_graph_builder(&mut r))
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
        self.generation_step_listeners
            .iter()
            .for_each(|l| (l)(InnerOuterGenerationStep::Linking));
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
            |_: InnerGraph, _: InnerGraph| vec![InterGraphEdge::FirstToSecond(0, 0)];
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
            |_: InnerGraph, _: InnerGraph| vec![InterGraphEdge::SecondToFirst(0, 0)];
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
