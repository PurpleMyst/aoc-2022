use std::fmt::Display;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut x: i16 = 1;
    let mut cycles: i16 = 0;
    let mut signal_strength: i16 = 0;

    let mut values = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut it = values.iter_mut();
    include_str!("input.txt").lines().for_each(|line| {
        if line == "noop" {
            *it.next().unwrap() = x;
            cycles += 1;
            if cycles == 20 || (cycles > 20 && (cycles - 20) % 40 == 0) {
                signal_strength += cycles * x;
            }
        } else {
            if matches!(cycles, 18 | 19) {
                let addition = (cycles + 19) / 20 * 20 * x;
                signal_strength += addition;
            }
            if cycles > 20 && matches!((cycles - 20) % 40, 38 | 39) {
                let addition = ((cycles - 20 + 39) / 40 * 40 + 20) * x;
                signal_strength += addition;
            }

            *it.next().unwrap() = x;
            *it.next().unwrap() = x;
            cycles += 2;
            x += line["addx ".len()..].parse::<i16>().unwrap();
        }
    });

    let mut it = values.into_iter().enumerate();
    let mut p2 = String::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT);
    for _y in 0..6 {
        p2.extend(
            it.by_ref().take(40)
            .map(|(clk, x)| if (x-1..=x+1).contains(&((clk % 40) as _)) {'█'} else { ' ' })
        );
        p2.push('\n');
    }

    (signal_strength, p2)
}
