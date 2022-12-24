#![feature(str_split_as_str)]
use std::fmt::Display;

use ahash::HashMap;
use itertools::Itertools;
use petgraph::prelude::*;

mod state;
use state::*;

fn solve_part<const PART2: bool>(initial_state: State<PART2>, flows: &[u16; 16], distances: &[u8; 256]) -> u16 {
    let mut states = binary_heap_plus::BinaryHeap::new_by_key(|state: &State<PART2>| state.relieved());
    let mut lower_bound = 0;

    states.push(initial_state);
    while let Some(state) = states.pop() {
        let upper_bound = state.upper_bound();
        if upper_bound < lower_bound {
            continue;
        }

        let old_len = states.len();
        states.extend(state.advance(flows, distances));
        if states.len() == old_len {
            if state.relieved() > lower_bound {
                lower_bound = state.relieved();
            }
        }
    }
    lower_bound
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
    let graph = graph.into_graph::<u8>();
    let start = graph.node_indices().find(|&idx| graph[idx] == "AA").unwrap();
    let graph = graph.map(|_, node_name| flow_rates[node_name], |_, &e| e);
    debug_assert!(graph.node_count() == 16);
    debug_assert_eq!(start.index(), 0);
    let total_flow: u16 = graph.node_weights().sum();

    // Convert the adjacency list into a 2x2 distance matrix via Floyd-Warshall
    let mut flows = [0; 16];
    let mut distances = [u8::MAX; 16 * 16];
    for i in 0..16 {
        flows[i] = *graph.node_weight(NodeIndex::new(i)).unwrap();
    }
    for edge in graph.edge_references() {
        distances[edge.source().index() * 16 + edge.target().index()] = *edge.weight();
        distances[edge.target().index() * 16 + edge.source().index()] = *edge.weight();
    }
    for k in 0..16 {
        distances[k * 16 + k] = 0;

        for i in 0..16 {
            for j in 0..16 {
                if let Some(result) = distances[i * 16 + k].checked_add(distances[k * 16 + j]) {
                    if distances[i * 16 + j] > result {
                        distances[i * 16 + j] = result;
                    }
                }
            }
        }
    }

    let p1 = solve_part::<false>(State::new(total_flow), &flows, &distances);
    let p2 = solve_part::<true>(State::new(total_flow), &flows, &distances);
    (p1, p2)
}
