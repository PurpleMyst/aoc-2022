
use std::fmt::Display;

fn parse_compartment(s: &str) -> u64 {
    let mut result = 0;
    s.bytes().for_each(|b| {
        debug_assert!(b.is_ascii_alphabetic());
        result |= if b.is_ascii_lowercase() {
            1 << (b - b'a')
        } else {
            1 << (b - b'A' + 26)
        };
    });
    result
}

fn parse_rucksack(s: &str) -> (u64, u64) {
    let (first, second) = s.split_at(s.len() / 2);
    (parse_compartment(first), parse_compartment(second))
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let elves = include_str!("input.txt").lines().map(parse_rucksack).collect::<Vec<_>>();
    let mut p1 = 0;
    let mut p2 = 0;

    for group in elves.chunks(3) {
        let group: [_; 3] = group.try_into().unwrap();
        for (l, r) in group {
            p1 += (l & r).trailing_zeros() + 1;
        }
        let group = group.map(|(l, r)| l | r);
        p2 += (group[0] & group[1] & group[2]).trailing_zeros() + 1;
    }

    (p1, p2)
}
