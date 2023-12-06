use std::{collections::hash_map::Entry, fmt::Display};

use ahash::HashMap;
use hibitset::{BitSet, BitSetLike};

const INITIAL_OFFSET: u32 = 100;
const SIDE_LENGTH: u32 = 256;

fn coord2idx((x, y): (u32, u32)) -> u32 {
    (y * SIDE_LENGTH + x)
}

fn idx2coord(idx: u32) -> (u32, u32) {
    (idx % SIDE_LENGTH, idx / SIDE_LENGTH)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut elves = BitSet::new();

    include_str!("input.txt").lines().enumerate().for_each(|(y, row)| {
        row.bytes().enumerate().for_each(|(x, b)| {
            if b == b'#' {
                elves.add(coord2idx((x as u32 + INITIAL_OFFSET, y as u32 + INITIAL_OFFSET)));
            }
        })
    });

    // north, south, west, east
    let mut directions: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut part1 = 0;
    let mut part2 = 0;

    let mut new_elves: HashMap<(u32, u32), ((u32, u32), bool)> = HashMap::default();

    for round in 0..1024 {
        let elves_to_move = (&elves)
            .iter()
            .map(idx2coord)
            .filter(|elf| {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let neighbor = (
                            elf.0.checked_add_signed(dx).unwrap(),
                            elf.1.checked_add_signed(dy).unwrap(),
                        );
                        if elves.contains(coord2idx(neighbor)) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<_>>();
        if elves_to_move.len() == 0 {
            part2 = round + 1;
            break;
        }

        for elf in elves_to_move {
            'dirloop: for direction in &directions {
                for delta in -1..=1 {
                    let neighbor = if direction.0 == 0 {
                        (
                            elf.0.checked_add_signed(delta).unwrap(),
                            elf.1.checked_add_signed(direction.1).unwrap(),
                        )
                    } else {
                        (
                            elf.0.checked_add_signed(direction.0).unwrap(),
                            elf.1.checked_add_signed(delta).unwrap(),
                        )
                    };
                    if elves.contains(coord2idx(neighbor)) {
                        continue 'dirloop;
                    }
                }
                let target = (
                    elf.0.checked_add_signed(direction.0).unwrap(),
                    elf.1.checked_add_signed(direction.1).unwrap(),
                );
                match new_elves.entry(target) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().1 = false;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((elf, true));
                    }
                }
                break;
            }
        }
        for (target, (elf, should)) in new_elves.drain() {
            if !should {
                continue;
            }
            elves.remove(coord2idx(elf));
            elves.add(coord2idx(target));
        }

        if round == 9 {
            let max_y = (&elves).iter().map(idx2coord).map(|(_, y)| y).max().unwrap();
            let max_x = (&elves).iter().map(idx2coord).map(|(x, _)| x).max().unwrap();
            let min_y = (&elves).iter().map(idx2coord).map(|(_, y)| y).min().unwrap();
            let min_x = (&elves).iter().map(idx2coord).map(|(x, _)| x).min().unwrap();
            part1 = (max_x - min_x + 1) as u16 * (max_y - min_y + 1) as u16 - (&elves).iter().count() as u16;
        }

        directions.rotate_left(1);
    }

    (part1, part2)
}
