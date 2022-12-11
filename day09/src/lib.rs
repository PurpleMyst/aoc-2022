use std::fmt::Display;

type Point = (i16, i16);

const PART2_KNOTS: usize = 10;

fn touching((hx, hy): Point, (tx, ty): Point) -> bool {
    hx.abs_diff(tx) <= 1 && hy.abs_diff(ty) <= 1
}

fn next_tail_pos((hx, hy): Point, (tx, ty): Point) -> Point {
    (tx + (hx - tx).signum(), ty + (hy - ty).signum())
}

fn simulate_part1() -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = fnv::FnvHashSet::default();
    visited.reserve(7168);
    visited.insert(tail);

    include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(dir, amount)| {
            let (dx, dy) = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0..amount.parse::<u8>().unwrap() {
                let new_head = (head.0 + dx, head.1 + dy);
                if !touching(tail, new_head) {
                    tail = head;
                }
                head = new_head;
                visited.insert(tail);
            }
        });

    visited.len()
}

fn simulate_part2() -> usize {
    let mut knots = [(0, 0); PART2_KNOTS];

    let mut visited = fnv::FnvHashSet::default();
    visited.reserve(3584);
    visited.insert(knots.last().copied().unwrap());

    include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(dir, amount)| {
            let (dx, dy) = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0..amount.parse::<u8>().unwrap() {
                let (hx, hy) = knots.first().copied().unwrap();
                knots[0] = (hx + dx, hy + dy);
                for i in 1..PART2_KNOTS {
                    if !touching(knots[i - 1], knots[i]) {
                        knots[i] = next_tail_pos(knots[i - 1], knots[i]);
                        } else {
                            break;
                        }
                }
                visited.insert(knots.last().copied().unwrap());
            }
        });

    visited.len()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (simulate_part1(), simulate_part2())
}
