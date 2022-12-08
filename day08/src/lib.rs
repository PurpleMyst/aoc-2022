use std::fmt::Display;

use bitvec::prelude::*;

const GRID_SIDE: usize = 99;

fn solve_part1(grid: &[i8]) -> usize {
    let mut visibility = bitarr![0; GRID_SIDE * GRID_SIDE];

    // horizontal lines
    for y in 0..GRID_SIDE {
        let mut tallest = -1;
        for x in 0..GRID_SIDE {
            let col = grid[y * GRID_SIDE + x];
            if col > tallest {
                visibility.set(y * GRID_SIDE + x, true);
                tallest = col;
            }
        }

        let mut tallest = -1;
        for x in (0..GRID_SIDE).rev() {
            let col = grid[y * GRID_SIDE + x];
            if col > tallest {
                visibility.set(y * GRID_SIDE + x, true);
                tallest = col;
            }
        }
    }

    // vertical lines
    for x in 0..GRID_SIDE {
        let mut tallest = -1;
        for y in 0..GRID_SIDE {
            let col = grid[y * GRID_SIDE + x];
            if col > tallest {
                visibility.set(y * GRID_SIDE + x, true);
                tallest = col;
            }
        }

        let mut tallest = -1;
        for y in (0..GRID_SIDE).rev() {
            let col = grid[y * GRID_SIDE + x];
            if col > tallest {
                visibility.set(y * GRID_SIDE + x, true);
                tallest = col;
            }
        }
    }

    visibility.count_ones()
}

fn solve_part2(grid: &[i8]) -> usize {
    let mut winner = 0;

    for y in 0..GRID_SIDE {
        for x in 0..GRID_SIDE {
            let height = grid[y * GRID_SIDE + x];
            let mut score = 1;
            let mut this_dir;

            this_dir = 0;
            for nx in x + 1..GRID_SIDE {
                this_dir += 1;
                if grid[y * GRID_SIDE + nx] >= height {
                    break;
                }
            }
            score *= this_dir;

            this_dir = 0;
            for nx in (0..x).rev() {
                this_dir += 1;
                if grid[y * GRID_SIDE + nx] >= height {
                    break;
                }
            }
            score *= this_dir;

            this_dir = 0;
            for ny in y + 1..GRID_SIDE {
                this_dir += 1;
                if grid[ny * GRID_SIDE + x] >= height {
                    break;
                }
            }
            score *= this_dir;

            this_dir = 0;
            for ny in (0..y).rev() {
                this_dir += 1;
                if grid[ny * GRID_SIDE + x] >= height {
                    break;
                }
            }
            score *= this_dir;

            winner = winner.max(score);
        }
    }
    winner
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let grid: Vec<i8> = include_str!("input.txt")
        .bytes()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| (ch - b'0') as i8)
        .collect();
    debug_assert_eq!(grid.len(), GRID_SIDE * GRID_SIDE);

    (solve_part1(&grid), solve_part2(&grid))
}
