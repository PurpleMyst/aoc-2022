use std::fmt::Display;

fn first_marker(chars: usize) -> usize {
    let bs = include_str!("input.txt").trim().as_bytes();

    let mut seen = [0usize; 26];
    let mut unique = 0;
    bs.iter().take(chars).for_each(|&b| {
        if seen[(b - b'a') as usize] == 0 {
            unique += 1;
        }
        seen[(b - b'a') as usize] += 1;
    });

    chars
        + 1
        + bs.windows(chars + 1)
            .position(|w| {
                let &[first, .., last] = w else {unreachable!()};
                seen[(first - b'a') as usize] -= 1;
                if seen[(first - b'a') as usize] == 0 {
                    unique -= 1;
                }
                if seen[(last - b'a') as usize] == 0 {
                    unique += 1;
                }
                seen[(last - b'a') as usize] += 1;
                unique == chars
            })
            .unwrap()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (first_marker(4), first_marker(14))
}
