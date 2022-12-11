use std::io::stdout;
use std::{collections::HashSet, fmt::Display};

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor,
    style::{Attribute, Color, Stylize},
    ExecutableCommand,
};

type Point = (i64, i64);

fn touching(head: Point, tail: Point) -> bool {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (head.0 + dx, head.1 + dy)))
        .any(|p| p == tail)
}

fn next_tail_pos((hx, hy): Point, (tx, ty): Point) -> Point {
    (tx + (hx - tx).signum(), ty + (hy - ty).signum())
}

fn simulate<const KNOT_AMOUNT: usize>() -> usize {
    let mut knots = [(0, 0); KNOT_AMOUNT];

    let mut visited = HashSet::new();
    visited.insert(knots.last().copied().unwrap());

    // stdout().execute(EnterAlternateScreen).unwrap();
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
                for i in 1..KNOT_AMOUNT {
                    if !touching(knots[i - 1], knots[i]) {
                        knots[i] = next_tail_pos(knots[i - 1], knots[i]);
                    }
                }
                visited.insert(knots.last().copied().unwrap());
            }
        });

    visited.len()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (simulate::<2>(), simulate::<10>())
}
