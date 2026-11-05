use std::collections::VecDeque;

use advent_of_code_2023::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2023::solution!(506_202);

const CUT_SIZE: usize = 3;

struct Graph {
    /// `(neighbor, arc)` per node. Arcs `2 * e` and `2 * e + 1` are the two directions of edge `e`, so `arc ^ 1` is the reverse arc.
    adjacency: Vec<Vec<(usize, usize)>>,
    arc_count: usize,
}

impl Graph {
    fn node<'input>(
        &mut self,
        indices: &mut HashMap<&'input str, usize>,
        name: &'input str,
    ) -> usize {
        *indices.entry(name).or_insert_with(|| {
            self.adjacency.push(Vec::new());
            self.adjacency.len() - 1
        })
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let arc = self.arc_count;

        self.adjacency[from].push((to, arc));
        self.adjacency[to].push((from, arc + 1));

        self.arc_count += 2;
    }
}

fn parse_input(input: &str) -> Result<Graph, &'static str> {
    let mut indices = HashMap::<&str, usize>::new();
    let mut graph = Graph {
        adjacency: Vec::new(),
        arc_count: 0,
    };

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (name, others) = line.split_once(':').ok_or("Missing : separator")?;

        let from = graph.node(&mut indices, name.trim());

        for other in others.split_whitespace() {
            let to = graph.node(&mut indices, other);

            graph.add_edge(from, to);
        }
    }

    Ok(graph)
}

enum Search {
    /// The search reached the sink and pushed one unit of flow along the path.
    Augmented,
    /// The search did not reach the sink. Holds the number of nodes reachable from the source.
    Blocked(usize),
}

#[derive(Clone)]
enum Visit {
    Unvisited,
    Source,
    Via { previous: usize, arc: usize },
}

struct FlowNetwork<'graph> {
    graph: &'graph Graph,
    /// Net flow per arc, in `-1..=1`. Every edge has capacity 1 in both directions.
    flow: Vec<i8>,
    /// How the current search reached each node.
    visits: Vec<Visit>,
    queue: VecDeque<usize>,
}

impl<'graph> FlowNetwork<'graph> {
    fn new(graph: &'graph Graph) -> Self {
        Self {
            graph,
            flow: vec![0; graph.arc_count],
            visits: vec![Visit::Unvisited; graph.adjacency.len()],
            queue: VecDeque::new(),
        }
    }

    /// Finds a shortest path with spare capacity from `source` to `sink` and pushes one unit of flow along it.
    fn augment(&mut self, source: usize, sink: usize) -> Search {
        self.visits.fill(Visit::Unvisited);
        self.queue.clear();

        self.visits[source] = Visit::Source;
        self.queue.push_back(source);

        let mut reachable = 1;

        while let Some(node) = self.queue.pop_front() {
            for &(neighbor, arc) in &self.graph.adjacency[node] {
                if !matches!(self.visits[neighbor], Visit::Unvisited) || self.flow[arc] >= 1 {
                    continue;
                }

                self.visits[neighbor] = Visit::Via {
                    previous: node,
                    arc,
                };
                reachable += 1;

                if neighbor == sink {
                    self.push_flow(sink);

                    return Search::Augmented;
                }

                self.queue.push_back(neighbor);
            }
        }

        Search::Blocked(reachable)
    }

    fn push_flow(&mut self, sink: usize) {
        let mut node = sink;

        while let Visit::Via { previous, arc } = self.visits[node] {
            self.flow[arc] += 1;
            self.flow[arc ^ 1] -= 1;

            node = previous;
        }
    }

    /// When exactly `CUT_SIZE` edge-disjoint paths connect `source` and `sink`, returns the size of the source's side of the cut.
    fn cut_side_size(&mut self, source: usize, sink: usize) -> Option<usize> {
        self.flow.fill(0);

        for _ in 0..CUT_SIZE {
            if let Search::Blocked(_) = self.augment(source, sink) {
                return None;
            }
        }

        match self.augment(source, sink) {
            Search::Augmented => None,
            Search::Blocked(reachable) => Some(reachable),
        }
    }
}

/// Returns the group sizes on both sides of the `CUT_SIZE`-edge cut.
fn split_groups(graph: &Graph) -> Option<(usize, usize)> {
    let source = 0;
    let node_count = graph.adjacency.len();

    let mut network = FlowNetwork::new(graph);

    // a sink on the source's side has more than `CUT_SIZE` edge-disjoint paths to the source
    (1..node_count)
        .find_map(|sink| network.cut_side_size(source, sink))
        .map(|size| (size, node_count - size))
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let graph = parse_input(input).unwrap();

        let (left, right) = split_groups(&graph).unwrap();

        PartSolution::USize(left * right)
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2023::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(506_202);
        }

        #[test]
        fn example() {
            test_example_part_1!(54);
        }
    }
}
