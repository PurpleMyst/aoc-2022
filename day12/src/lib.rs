use std::{collections::VecDeque, fmt::Display};

use petgraph::{
    prelude::*,
    visit::{VisitMap, Visitable},
};

fn height(ch: u8) -> u8 {
    (match ch {
        b'S' => b'a',
        b'E' => b'z',
        _ => ch,
    }) - b'a'
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // Calculate the grid size and create a Graph with the needed capacity
    let input = include_str!("input.txt").trim();
    let width = input.find('\n').unwrap();
    let grid_size = width * (1 + input.bytes().filter(|&ch| ch == b'\n').count());
    let mut graph: Graph<(), (), Directed, u32> = Graph::with_capacity(grid_size, 4 * grid_size);

    // Load the heights into a grid, saving the start, goal and all the valleys we find.
    let mut start = None;
    let mut end = None;
    let mut lows = graph.visit_map();
    lows.grow(grid_size);
    let grid_data: Vec<(u8, NodeIndex<_>)> = input
        .bytes()
        .filter(|ch| matches!(ch, b'a'..=b'z' | b'S' | b'E'))
        .map(|ch| (ch, graph.add_node(())))
        .inspect(|(ch, idx)| match ch {
            b'S' => start = Some(idx.clone()),
            b'E' => end = Some(idx.clone()),
            b'a' => {
                lows.visit(idx.clone());
            }
            _ => {}
        })
        .map(|(ch, idx)| (height(ch), idx))
        .collect();
    let grid = grid::Grid::from_vec(grid_data, width);
    let start = start.unwrap();
    let end = end.unwrap();

    // Load the graph by calculating on which nodes we can step onto from each node.
    // We do this in _reverse_ order, as in, we care about which nodes we can step "down" to, not
    // which ones we can stop "up" to, as starting the search from the end is more efficient.
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let (cur_h, cur_id) = grid.get(y, x).copied().unwrap();

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if (dx == 0 && dy == 0) || (dx != 0 && dy != 0) {
                        continue;
                    }

                    if let Some((neigh_h, neigh_id)) = usize::try_from(x as isize + dx)
                        .ok()
                        .zip(usize::try_from(y as isize + dy).ok())
                        .and_then(|(x, y)| grid.get(y, x))
                        .cloned()
                    {
                        if neigh_h <= cur_h || neigh_h == cur_h + 1 {
                            graph.add_edge(neigh_id, cur_id, ());
                        }
                    }
                }
            }
        }
    }

    // Now run a BFS from the end, searching for the first 'a' we encounter (which must be the
    // closest as the graph is unweighted) and then the start.
    let mut stack = VecDeque::with_capacity(grid_size);
    let mut visited = graph.visit_map();
    let mut p1 = i32::MAX;
    let mut p2 = i32::MAX;
    stack.push_back((end, 0));
    while let Some((node, steps)) = stack.pop_front() {
        // ASSUMPTION: The start node is farther than the closest 'a'.
        if node == start {
            p1 = steps;
            break;
        }
        if lows.is_visited(&node) && p2 == i32::MAX {
            p2 = steps;
        }
        stack.extend(
            graph
                .neighbors(node)
                .filter(|neighbor| visited.visit(neighbor.clone()))
                .map(|neighbor| (neighbor, steps + 1)),
        );
    }

    (p1, p2)
}
