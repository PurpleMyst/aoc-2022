use std::fmt::Display;

fn first_marker(chars: usize) -> usize {
    include_str!("input.txt")
        .trim()
        .as_bytes()
        .windows(chars)
        .position(|w| {
            w.iter()
                .enumerate()
                .all(|(i, a)| w.iter().skip(i + 1).all(|b| a != b))
        })
        .unwrap()
        + chars
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (first_marker(4), first_marker(14))
}
