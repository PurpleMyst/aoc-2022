use std::fmt::Display;

fn first_marker(chars: usize) -> usize {
    chars
        + include_str!("input.txt")
            .trim()
            .as_bytes()
            .windows(chars)
            .position(|w| {
                let mut seen = 0u32;
                w.iter().all(|&ch| {
                    let mask = 1 << (ch - b'a');
                    if seen & mask == 0 {
                        seen |= mask;
                        true
                    } else {
                        false
                    }
                })
            })
            .unwrap()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (first_marker(4), first_marker(14))
}
