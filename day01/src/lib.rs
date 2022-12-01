use std::fmt::Display;

const TOP_N: usize = 3;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut biggest = [0; TOP_N];

    include_str!("input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(|n| n.parse::<u64>().unwrap())
                .sum()
        })
        .for_each(|elf| {
            if let Some(i) = biggest.iter().position(|&n| elf >= n) {
                biggest[i..].rotate_right(1);
                biggest[i] = elf;
            }
        });

    (biggest[0], biggest.into_iter().sum::<u64>())
}
