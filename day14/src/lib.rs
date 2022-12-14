use std::{collections::HashSet, fmt::Display, ops::RangeInclusive};

type Point = (i64, i64);

const SAND_SOURCE: Point = (500, 0);

fn range(a: i64, b: i64) -> RangeInclusive<i64> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn is_occupied(walls: &HashSet<Point>, death_y: i64, p: &Point) -> bool {
    walls.contains(&p) || p.1 == death_y + 2
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut walls: HashSet<Point> = HashSet::new();

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

    let mut p1 = None;
    let mut dropped = 0;
    loop {
        let mut sand = SAND_SOURCE;
        loop {
            if sand.1 > death_y && p1.is_none() {
                p1 = Some(dropped);
            }

            let candidates = [
                (sand.0, sand.1 + 1),
                (sand.0 - 1, sand.1 + 1),
                (sand.0 + 1, sand.1 + 1),
            ];
            if let Some(next) = candidates
                .into_iter()
                .find(|p| !is_occupied(&walls, death_y, p))
            {
                sand = next;
            } else {
                break;
            }
        }
        dropped += 1;
        walls.insert(sand);
        if sand == SAND_SOURCE {
            break;
        }
    }

    (p1.unwrap(), dropped)
}
