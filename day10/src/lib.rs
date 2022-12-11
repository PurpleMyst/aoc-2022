use std::fmt::Display;

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

struct Screen([i8; SCREEN_WIDTH * SCREEN_HEIGHT]);

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.0.into_iter().enumerate();
        for _y in 0..SCREEN_HEIGHT {
            // If, at each clock cycle, the sprite overlapped with the pixel currently being drawn
            // then we should color in that pixel, otherwise leave it blank.
            it.by_ref().take(SCREEN_WIDTH).try_for_each(|(clk, x)| {
                write!(
                    f,
                    "{}",
                    if (x - 1..=x + 1).contains(&((clk % 40) as i8)) {
                        '█'
                    } else {
                        ' '
                    }
                )
            })?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // Execute the program, storing the values in each clock cycle into an array.
    let mut x: i8 = 1;
    let mut values = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut it = values.iter_mut();
    include_str!("input.txt").lines().for_each(|line| {
        // It always takes at least one clock cycle to execute an instruction, during which the
        // value of X will remain the same. Therefore, the next clock cycle's values of X will be
        // the curren tone.
        *it.next().unwrap() = x;
        // Then, if this instruction is an addx, we'll spend one more cycle doing some internal
        // work to increment the value of X, during which the value of X will not vary.
        if line != "noop" {
            *it.next().unwrap() = x;
            x += line["addx ".len()..].parse::<i8>().unwrap();
        }
    });

    // Compute the signal strength by taking every (20 + 40n)th value
    let signal_strength: i16 = values
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(clk, &x)| (clk + 1) as i16 * x as i16)
        .sum();

    // Return the signal strength + the Screen, which knows how to Display itself.
    (signal_strength, Screen(values))
}
