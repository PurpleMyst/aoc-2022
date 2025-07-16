use std::fmt::Display;

use super::{parse_input, Instruction};

const SIDE: usize = 50;

const OPEN: u8 = b'.';

// Directions in (dx, dy) form.
const UP: (i8, i8) = (0, -1);
const DOWN: (i8, i8) = (0, 1);
const LEFT: (i8, i8) = (-1,0);
const RIGHT: (i8, i8) = (1,0);

pub(super) fn solve_part() -> impl Display {
    let (instructions, map) = parse_input();
    let mut regions: [_; 6] = std::array::from_fn(|_|grid::Grid::<u8>::new(SIDE, SIDE) );

    for y in 0..SIDE {
        for x in 0..SIDE {
            for (r, region) in regions.iter_mut().enumerate() {
                let map_y = y + match r+1 {
                    1 => 0,
                    2 => 0,
                    3 => SIDE,
                    4 => 2 * SIDE,
                    5 => 2 * SIDE,
                    6 => 3 * SIDE,
                    _ => unreachable!(),
                };
                let map_x = x + SIDE.checked_add_signed(match r+1 {
                    1 => 0,
                    2 => SIDE as isize,
                    3 => 0,
                    4 => 0,
                    5 => -(SIDE as isize),
                    6 => -(SIDE as isize),
                    _ => unreachable!(),
                }).unwrap();
                region[(y, x)] = map[(map_y, map_x)];
            }
        }
    }

    let mut y = 0i8;
    let mut x = regions[0].iter_row(0).position(|b| *b == OPEN).unwrap() as i8;
    let mut r = 0usize;

    let mut dx = 1;
    let mut dy = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    // Compute the target position, considered as a triple which incorporates the
                    // region (i.e. face).
                    let (ty, tx, tr) = if (0..SIDE as i8).contains(&(x + dx)) && (0..SIDE as i8).contains(&(y + dy)) {
                        (x + dx, y + dy, r)
                    } else if x + dx == SIDE as i8 {
                        // x == SIDE-1; used in place of the latter due to type (i8)
                        match r+1 {
                            1 => (0, y, 2), // 0° rotation
                            2 => (x, x - y, 4), // 180° rotation
                            3 => (y, x, 2), // 90° CCW rotation
                            4 => (x, x - y, 4), // 180° rotation
                            5 => (0, y, 4), // 0° rotation
                            6 => (y, x, 4), // 90° CCW rotation
                            _ => unreachable!(),
                        }
                    } else if x + dx == -1 {
                        // x == 0;
                        match r + 1 {
                            1 => (x, SIDE as i8 - 1 - y, 5), // 180° rotation
                            2 => (SIDE as i8 - 1, y, 1), // 0° rotation
                            3 => (x, y, 5), // 90° CW rotation
                            _ => unreachable!(),
                        }
                    } else {
                        todo!()
                    };
                }
            },

            Instruction::TurnCW => {
                (dx, dy) = match (dx, dy) {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => unreachable!(),
                }
            }
            Instruction::TurnCCW => {
                for _ in 0..3 {
                    (dx, dy) = match (dx, dy) {
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        (0, -1) => (1, 0),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }


    "TODO"
}
