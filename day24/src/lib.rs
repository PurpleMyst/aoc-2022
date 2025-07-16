use std::fmt::Display;

mod part1;
mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let part1 = part1::solve_part();
    let part2 = part2::solve_part();

    (part1, part2)
}

