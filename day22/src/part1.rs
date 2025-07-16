use super::Instruction;

use super::OPEN;

use super::parse_input;

pub(crate) fn solve_part() -> usize {
    let (instructions, map) = parse_input();

    let mut y: usize = 0;
    let mut x: usize = map.iter_row(0).enumerate().find(|(_, b)| **b == OPEN).unwrap().0;

    let mut dy: isize = 0;
    let mut dx: isize = 1;

    for instruction in dbg!(instructions) {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    let new_y = y.wrapping_add_signed(dy);
                    let new_x = x.wrapping_add_signed(dx);

                    let ((new_y, new_x), cell) = map
                        .get(new_y, new_x)
                        .filter(|cell| **cell != b' ')
                        .map(|cell| ((new_y, new_x), cell))
                        .or_else(|| {
                            let mut next_y = y;
                            let mut next_x = x;

                            while map.get(next_y, next_x).filter(|cell| **cell != b' ').is_some() {
                                next_y = next_y.wrapping_add_signed(-dy);
                                next_x = next_x.wrapping_add_signed(-dx);
                            }

                            next_y = next_y.wrapping_add_signed(dy);
                            next_x = next_x.wrapping_add_signed(dx);

                            Some(((next_y, next_x), map.get(next_y, next_x)?))
                        })
                        .unwrap();

                    if *cell == OPEN {
                        y = new_y;
                        x = new_x;
                    } else {
                        break;
                    }
                }
            }

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

    1000 * (y + 1)
        + 4 * (x + 1)
        + match (dx, dy) {
            (1, 0) => 0,
            (-1, 0) => 2,
            (0, -1) => 3,
            (0, 1) => 1,
            _ => unreachable!(),
        }
}

