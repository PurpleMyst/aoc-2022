use std::fmt::Display;

pub(crate) const WIDTH: usize = 120;

pub(crate) const HEIGHT: usize = 25;

pub(crate) const START: (i8, i8) = (0, -1);

pub(crate) const END: (i8, i8) = (WIDTH as i8 - 1, HEIGHT as i8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct State {
    pub(crate) left: [u128; HEIGHT],
    pub(crate) right: [u128; HEIGHT],
    pub(crate) up: [u128; WIDTH],
    pub(crate) down: [u128; WIDTH],
    pub(crate) pos: (i8, i8),
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in -1..=HEIGHT as isize {
            for x in -1..=WIDTH as isize {
                if (x, y) == (self.pos.0 as isize, self.pos.1 as isize) {
                    write!(f, "\x1b[35;6;1mE\x1b[0m")?;
                } else if (x as i8, y as i8) == START || (x as i8, y as i8) == END {
                    write!(f, ".")?;
                } else if !((0..WIDTH as isize).contains(&x) && (0..HEIGHT as isize).contains(&y)) {
                    write!(f, "█")?;
                } else if self.left[y as usize] & (1 << x) != 0 {
                    write!(f, "\x1b[31m<\x1b[0m")?;
                } else if self.right[y as usize] & (1 << x) != 0 {
                    write!(f, "\x1b[32m>\x1b[0m")?;
                } else if self.up[x as usize] & (1 << y) != 0 {
                    write!(f, "\x1b[33m^\x1b[0m")?;
                } else if self.down[x as usize] & (1 << y) != 0 {
                    write!(f, "\x1b[34mv\x1b[0m")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub(crate) fn rotate_n_bits_left(value: u128, n: usize) -> u128 {
    (value << 1) | (value >> (n - 1))
}

pub(crate) fn rotate_n_bits_right(value: u128, n: usize) -> u128 {
    (value >> 1) | (value << (n - 1))
}

impl State {
    pub(crate) fn advance_blizzards(mut self) -> Self {
        self.down = self.down.map(|d| rotate_n_bits_left(d, HEIGHT));
        self.up = self.up.map(|u| rotate_n_bits_right(u, HEIGHT));
        self.left = self.left.map(|l| rotate_n_bits_right(l, WIDTH));
        self.right = self.right.map(|r| rotate_n_bits_left(r, WIDTH));
        self
    }

    pub(crate) fn is_safe(&self, pos: (i8, i8)) -> bool {
        if pos == START || pos == END {
            return true;
        }
        if !(0..WIDTH as i8).contains(&pos.0) || !(0..HEIGHT as i8).contains(&pos.1) {
            return false;
        }
        let x = pos.0 as usize;
        let y = pos.1 as usize;
        !(self.left[y] & (1 << x) != 0
            || self.right[y] & (1 << x) != 0
            || self.up[x] & (1 << y) != 0
            || self.down[x] & (1 << y) != 0)
    }

    pub(crate) fn done(&self) -> bool {
        self.pos == END
    }

    pub(crate) fn next(self) -> impl Iterator<Item = Self> {
        let next_blizzards = self.advance_blizzards();

        [
            self.pos,                     // wait
            (self.pos.0 + 1, self.pos.1), // right
            (self.pos.0 - 1, self.pos.1), // left
            (self.pos.0, self.pos.1 + 1), // down
            (self.pos.0, self.pos.1 - 1), // up
        ]
        .into_iter()
        .filter(move |&pos| next_blizzards.is_safe(pos))
        .map(move |pos| {
            let mut next_state = next_blizzards;
            next_state.pos = pos;
            next_state
        })
    }
}

pub(crate) fn solve_part() -> u64 {
    let mut initial_state = State {
        pos: START,
        left: [0; HEIGHT],
        right: [0; HEIGHT],
        up: [0; WIDTH],
        down: [0; WIDTH],
    };

    include_str!("input.txt")
        .lines()
        .skip(1)
        .take(HEIGHT)
        .enumerate()
        .for_each(|(y, row)| {
            row.chars()
                .skip(1)
                .take(WIDTH)
                .enumerate()
                .for_each(|(x, cell)| match cell {
                    '>' => initial_state.right[y] |= 1 << x,
                    '<' => initial_state.left[y] |= 1 << x,
                    'v' => initial_state.down[x] |= 1 << y,
                    '^' => initial_state.up[x] |= 1 << y,
                    _ => {}
                })
        });

    let (_, part1) = pathfinding::prelude::astar(
        &initial_state,
        |state| state.next().map(|next_state| (next_state, 1)),
        |state| state.pos.0.abs_diff(END.0) as u64 + state.pos.1.abs_diff(END.1) as u64,
        |state| state.done(),
    )
    .unwrap();

    part1
}

