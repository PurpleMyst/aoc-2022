use std::fmt::Display;

type Pair = (u8, u8);

fn parse_range(r: &str) -> Pair {
    let (s, e) = r.split_once('-').unwrap();
    (s.parse().unwrap(), e.parse().unwrap())
}

fn contains(container: Pair, contained: Pair) -> bool {
    container.0 <= contained.0 && container.1 >= contained.1
}

fn overlap(first: Pair, second: Pair) -> bool {
    first.0 <= second.1 && first.1 >= second.0
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut p1 = 0;
    let mut p2 = 0;

    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (parse_range(l), parse_range(r))
        })
        .for_each(|(l, r)| {
            if contains(l, r) || contains(r, l) {
                p1 += 1;
            }
            if overlap(l, r) || overlap(r, l) {
                p2 += 1;
            }
        });

    (p1, p2)
}
