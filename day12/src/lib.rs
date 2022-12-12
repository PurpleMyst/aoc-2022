use std::fmt::Display;

use petgraph::{algo::dijkstra, prelude::*};

fn height(ch: u8) -> u8 {
    (match ch {
        b'S' => b'a',
        b'E' => b'z',
        b'a'..=b'z' => ch,
        _ => unreachable!(),
    }) - b'a'
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").trim();
    let width = input.find('\n').unwrap();
    let grid_size = width * input.bytes().filter(|&ch| ch == b'\n').count();
    let mut graph: Graph<(), (), Directed, u32> = Graph::with_capacity(grid_size, 4 * grid_size);

    let mut start = None;
    let mut end = None;
    let mut lows = Vec::new();
    let grid_data: Vec<(u8, NodeIndex<_>)> = input
        .bytes()
        .filter(|ch| matches!(ch, b'a'..=b'z' | b'S' | b'E'))
        .map(|ch| (ch, graph.add_node(())))
        .inspect(|(ch, idx)| match ch {
            b'S' => start = Some(idx.clone()),
            b'E' => end = Some(idx.clone()),
            b'a' => lows.push(idx.clone()),
            _ => {}
        })
        .map(|(ch, idx)| (height(ch), idx))
        .collect();
    let grid = grid::Grid::from_vec(grid_data, width);
    let start = start.unwrap();
    let end = end.unwrap();

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

    let costs = dijkstra(&graph, end, None, |_| 1);
    let p1 = costs[&start];
    let p2 = lows
        .into_iter()
        .flat_map(|idx| costs.get(&idx).copied())
        .min()
        .unwrap();

    (p1, p2)
}
