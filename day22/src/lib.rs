#![allow(dead_code)]
use std::{fmt::Display, iter::repeat_n, mem::swap};

const OPEN: u8 = b'.';

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnCW,
    TurnCCW,
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut n = 0;
    let mut res = Vec::new();
    for b in instructions.bytes() {
        if b.is_ascii_digit() {
            n = 10 * n + (b - b'0') as usize;
        } else {
            if n != 0 {
                res.push(Instruction::Move(n));
                n = 0;
            }
            match b {
                b'R' => res.push(Instruction::TurnCW),
                b'L' => res.push(Instruction::TurnCCW),
                _ => unreachable!(),
            }
        }
    }
    if n != 0 {
        res.push(Instruction::Move(n));
    }
    res
}

fn parse_input() -> (Vec<Instruction>, grid::Grid<u8>) {
    let input = include_str!("sample_input_sides.txt");

    let mut it = input.lines();

    let instructions = parse_instructions(it.next_back().unwrap());
    assert_eq!(it.next_back().unwrap(), "");

    let width = it.clone().map(|line| line.len()).max().unwrap();

    let map = grid::Grid::from(
        it.map(|line| {
            let mut row = line.bytes().collect::<Vec<_>>();
            row.extend(repeat_n(b' ', width - row.len()));
            row
        })
        .collect::<Vec<_>>(),
    );
    (instructions, map)
}

mod part1;
mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // let part1 = part1::solve_part();
    let part1 = "SKIPPED";
    let part2 = part2::solve_part();

    (part1, part2)
}

