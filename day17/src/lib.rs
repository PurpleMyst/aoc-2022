use std::fmt::{Debug, Display};

use ahash::{HashMap, HashMapExt};
use itertools::{EitherOrBoth, Itertools};

const PIECE_SEQUENCE: [[u8; 4]; 5] = [
    [0b00111100, 0, 0, 0],
    [0b00010000, 0b00111000, 0b00010000, 0],
    [0b00111000, 0b00001000, 0b00001000, 0],
    [0b00100000, 0b00100000, 0b00100000, 0b00100000],
    [0b00110000, 0b00110000, 0, 0],
];

const PART2_PIECES: usize = 1_000_000_000_000;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Piece {
    rows: [u8; 4],
    bottom_y: usize,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Piece's bottom located at y={}", self.bottom_y)?;
        for row in self.rows.iter().rev() {
            writeln!(f, "{row:08b}")?;
        }
        Ok(())
    }
}

impl Piece {
    fn new(rows: [u8; 4], y: usize) -> Self {
        Self { rows, bottom_y: y }
    }

    fn move_right(&mut self, board: &[u8]) {
        let mut new_rows = self.rows;
        for z in new_rows.iter_mut().zip_longest(board[self.bottom_y..].iter()) {
            let (row, board_row) = match z {
                EitherOrBoth::Both(l, r) => (l, *r),
                EitherOrBoth::Left(l) => (l, 0),
                EitherOrBoth::Right(_) => break,
            };
            debug_assert_eq!(*row & 1, 0);
            let new_row = *row >> 1;
            if new_row & 1 != 0 || board_row & new_row != 0 {
                return;
            }
            *row = new_row;
        }
        self.rows = new_rows;
    }

    fn move_left(&mut self, board: &[u8]) {
        let mut new_rows = self.rows;
        for (row, board_row) in new_rows.iter_mut().zip(board[self.bottom_y..].iter()) {
            if *row & (0b1000_0000) != 0 {
                return;
            }
            let new_row = *row << 1;
            if new_row & board_row != 0 {
                return;
            }
            *row = new_row;
        }
        self.rows = new_rows;
    }

    fn move_down(&mut self, board: &mut [u8]) -> bool {
        if self.bottom_y != 0
            && board[self.bottom_y - 1..]
                .iter()
                .zip(self.rows.iter())
                .all(|(dst, src)| dst & src == 0)
        {
            self.bottom_y -= 1;
            true
        } else {
            debug_assert!(board[self.bottom_y..].len() >= self.rows.len());
            board[self.bottom_y..]
                .iter_mut()
                .zip(self.rows.iter())
                .for_each(|(dst, src)| {
                    debug_assert!((*dst & src) == 0);
                    *dst ^= src;
                });
            false
        }
    }
}

fn height(board: &[u8]) -> usize {
    board.iter().rev().skip_while(|row| row.count_ones() == 0).count()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut board = Vec::<u8>::new();
    let mut moves = include_str!("input.txt").trim().chars().map(Direction::from).cycle();

    let mut start_y = 3;
    for _y in 0..=10 {
        board.push(0);
    }

    let mut cleared_rows = 0;
    let mut seen: HashMap<(Vec<u8>, [u8; 4]), (usize, usize)> = HashMap::new();
    let mut iterations = 0;
    let mut remainder = None;
    let mut p1 = 0;
    for &piece_data in PIECE_SEQUENCE.iter().cycle() {
        iterations += 1;
        if iterations == 2022 {
            p1 = cleared_rows + height(&board);
        }

        if let Some(remainder) = &mut remainder {
            if *remainder == 0 {
                break;
            }
            *remainder -= 1;
        } else if let Some(prev) = seen.insert((board.clone(), piece_data), (iterations, cleared_rows)) {
            let repeats_every = iterations - prev.0;
            let gains = cleared_rows - prev.1;
            let remaining = PART2_PIECES - iterations;
            remainder = Some(remaining % repeats_every);
            cleared_rows += gains * (remaining / repeats_every);
        }

        while board[start_y - 3].count_ones() != 0 {
            board.push(0);
            start_y += 1;
        }

        let mut piece = Piece::new(piece_data, start_y);
        for move_ in moves.by_ref() {
            match move_ {
                Direction::Left => piece.move_left(&board),
                Direction::Right => piece.move_right(&board),
            }
            if !piece.move_down(&mut board) {
                break;
            }
        }

        if board[piece.bottom_y] == 0b1111_1110 {
            board.drain(..piece.bottom_y + 1);
            cleared_rows += piece.bottom_y + 1;
            start_y -= piece.bottom_y + 1;
        }
    }

    (p1, cleared_rows + height(&board))
}
