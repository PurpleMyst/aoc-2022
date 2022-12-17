#![feature(str_split_as_str)]
use std::fmt::Display;

use ahash::HashMap;
use itertools::Itertools;
use petgraph::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
struct Opened(u16);

impl Opened {
    fn update(self, idx: NodeIndex) -> Self {
        Self(self.0 | (1 << idx.index()))
    }

    fn contains(self, idx: NodeIndex) -> bool {
        (self.0 & (1 << idx.index())) != 0
    }
}

trait ProblemState: Sized {
    fn initial(position: NodeIndex) -> Self;
    fn time_left(&self) -> u8;
    fn per_second(&self) -> u16;
    fn total(&self) -> u16;
    fn opened(&self) -> Opened;
    fn advance(self, graph: &UnGraph<u16, u8>, states: &mut Vec<Self>);
}

mod part1;
mod part2;

fn solve_part<PS: ProblemState>(start: NodeIndex, graph: &UnGraph<u16, u8>) -> u16 {
    let mut states = Vec::new();
    let mut problem_lower_bound = 0;
    let total_flow_rate: u16 = graph.node_weights().sum();
    states.push(PS::initial(start));
    while let Some(state) = states.pop() {
        // Calculate what we could get if everything was open and we stood still from this point
        // onwards
        let upper_bound = state.total() + total_flow_rate * u16::from(state.time_left());

        // If that's less than our lower bound, this state isn't worth exploring.
        if upper_bound < problem_lower_bound {
            continue;
        }

        // Otherwise, if our upper bound corresponds to our final total, consider this a single candidate solution.
        if state.time_left() == 0 || state.per_second() == total_flow_rate {
            if upper_bound > problem_lower_bound {
                eprint!("\x1b[34;1m{upper_bound:4} \x1b[0m");
            }
            problem_lower_bound = problem_lower_bound.max(upper_bound);
            continue;
        }

        // Advance this state if it wasn't a single candidate solution yet.
        state.advance(graph, &mut states);
    }
    eprintln!();
    problem_lower_bound
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // Load in edges + flow rates
    let mut graph = UnGraphMap::new();
    let mut flow_rates: HashMap<&str, u16> = HashMap::default();
    for line in include_str!("input.txt").lines() {
        let mut words = line.split(' ');
        let valve = words.nth(1).unwrap();
        let flow_rate: u16 = words
            .nth(2)
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .strip_suffix(';')
            .unwrap()
            .parse()
            .unwrap();
        let _ = words.nth(3).unwrap();
        let passages = words.as_str().split(", ");
        flow_rates.insert(valve, flow_rate);
        for destination in passages {
            graph.add_edge(valve, destination, 1);
        }
    }

    // Simplify the graph: Nodes with zero flow rates are "just roads".
    while let Some((&to_remove, _)) = flow_rates.iter().filter(|&(&k, _)| k != "AA").find(|&(_, &v)| v == 0) {
        let neighbors: Vec<&str> = graph.neighbors(to_remove).collect();
        for (n1, n2) in neighbors.into_iter().tuple_combinations() {
            let &w1 = graph.edge_weight(n1, to_remove).unwrap();
            let &w2 = graph.edge_weight(to_remove, n2).unwrap();
            graph.add_edge(n1, n2, w1 + w2);
        }
        graph.remove_node(to_remove);
        flow_rates.remove(&to_remove);
    }

    // Convert the graphmap into an adjacency list which is must faster.
    let graph = graph.into_graph();
    let start = graph.node_indices().find(|&idx| graph[idx] == "AA").unwrap();
    let graph = graph.map(|_, node_name| flow_rates[node_name], |_, &e| e);
    debug_assert!(graph.node_count() <= 16);

    (solve_part::<part1::State>(start, &graph), 0)
}
