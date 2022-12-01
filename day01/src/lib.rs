use std::fmt::Display;

const TOP_N: usize = 3;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut p1 = 0;
    let mut p2 = [0; TOP_N];

    include_str!("input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(|n| n.parse::<u64>().unwrap())
                .sum()
        })
        .for_each(|elf| {
            p1 = p1.max(elf);
            if let Some(i) = p2.iter().position(|&n| elf >= n) {
                p2[i..].rotate_right(1);
                p2[i] = elf;
            }
        });

    (p1, p2.into_iter().sum::<u64>())
}
