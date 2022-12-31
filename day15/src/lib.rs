use std::{fmt::Display, ops::Bound, str::FromStr};

use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use scan_fmt::scan_fmt;
use unbounded_interval_tree::interval_tree::IntervalTree;

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
struct Sensor {
    location: Point,
    beacon: Point,
}

fn unwrap_bound(bound: &Bound<i64>) -> i64 {
    match bound {
        Bound::Included(n) => *n,
        Bound::Excluded(_) | Bound::Unbounded => unreachable!("Bounds should just be inclusive!"),
    }
}

impl FromStr for Sensor {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (location_x, location_y, beacon_x, beacon_y) = scan_fmt!(
            s,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i64,
            i64,
            i64,
            i64
        )?;
        Ok(Sensor {
            location: (location_x, location_y),
            beacon: (beacon_x, beacon_y),
        })
    }
}

fn manhattan_distance(a: Point, b: Point) -> u64 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn do_solve(input: &str, part1_row: i64, search_space: usize) -> (usize, i64) {
    let sensors = input.lines().map(|line| line.parse::<Sensor>().unwrap());
    let mut trees = vec![IntervalTree::default(); search_space];
    let mut beacons = ahash::HashSet::default();

    sensors.for_each(|sensor| {
        let d = manhattan_distance(sensor.location, sensor.beacon) as i64;
        for r in 0..=d {
            let start = sensor.location.0 - r;
            let end = sensor.location.0 + r;
            let y_offset = d - r;
            for y in [sensor.location.1 + y_offset, sensor.location.1 - y_offset] {
                if let Some(tree) = usize::try_from(y).ok().and_then(|y| trees.get_mut(y)) {
                    tree.insert(start..=end);
                }
            }
        }

        if sensor.beacon.1 == part1_row {
            beacons.insert(sensor.beacon.0);
        }
    });

    let (p1_min, p1_max) = trees[part1_row as usize]
        .iter()
        .flat_map(|(a, b)| [unwrap_bound(a), unwrap_bound(b)])
        .minmax()
        .into_option()
        .unwrap();
    let p1 = (p1_min..=p1_max)
        .into_par_iter()
        .filter(|x| trees[part1_row as usize].contains_point(&(*x as _)))
        .filter(|x| !beacons.contains(x))
        .count();

    let (p2_x, p2_y) = trees
        .into_iter()
        .enumerate()
        .find_map(|(y, tree)| {
            let range = 0..=search_space as i64;
            let mut v = tree.get_interval_difference(&range);
            let x = match v.pop()? {
                (Bound::Excluded(a), Bound::Excluded(b)) => (a + b) / 2,
                otherwise => unreachable!("{otherwise:?}"),
            };
            Some((x, y as i64))
        })
        .unwrap();
    let p2 = p2_x * 4_000_000 + p2_y;

    (p1, p2)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    do_solve(include_str!("input.txt"), 2_000_000, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(do_solve(include_str!("sample_input.txt"), 10, 20), (26, 56_000_011));
    }
}
