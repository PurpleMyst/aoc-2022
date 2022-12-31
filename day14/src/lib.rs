use std::{collections::HashSet, fmt::Display, ops::RangeInclusive};

use ahash::RandomState;

type Point = (u16, u16);
type PointSet = HashSet<Point, RandomState>;

const SAND_SOURCE: Point = (500, 0);

fn range(a: u16, b: u16) -> RangeInclusive<u16> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn solve_part2(walls: &PointSet, death_y: u16) -> usize {
    let mut stack = vec![SAND_SOURCE];
    let mut visited = PointSet::default();
    while let Some((x, y)) = stack.pop() {
        stack.extend(
            [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .into_iter()
                .filter(|&(_, y)| y < death_y + 2)
                .filter(|p| !walls.contains(p))
                .filter(|&p| visited.insert(p)),
        )
    }
    1 + visited.len()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut walls = PointSet::default();

    include_str!("input.txt").lines().for_each(|line| {
        let points = line.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        });
        points.reduce(|prev, cur| {
            if prev.0 == cur.0 {
                let x = prev.0;
                let ys = range(prev.1, cur.1);
                ys.for_each(|y| {
                    walls.insert((x, y));
                })
            } else {
                let xs = range(prev.0, cur.0);
                let y = prev.1;
                xs.for_each(|x| {
                    walls.insert((x, y));
                })
            }
            cur
        });
    });

    let &death_y = walls.iter().map(|(_, y)| y).max().unwrap();

    let p2 = solve_part2(&walls, death_y);

    let mut p1 = 0;
    'outer: loop {
        let mut sand = SAND_SOURCE;
        loop {
            if sand.1 >= death_y {
                break 'outer;
            }

            let candidates = [(sand.0, sand.1 + 1), (sand.0 - 1, sand.1 + 1), (sand.0 + 1, sand.1 + 1)];
            if let Some(next) = candidates.into_iter().find(|p| !walls.contains(p)) {
                sand = next;
            } else {
                break;
            }
        }
        p1 += 1;
        walls.insert(sand);
    }

    (p1, p2)
}
