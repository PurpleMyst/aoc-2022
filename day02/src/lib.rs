use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

use Move::*;
use Outcome::*;

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!(),
        })
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        })
    }
}

impl Outcome {
    fn value(self) -> u64 {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

impl Move {
    fn outcome(self, other: Move) -> Outcome {
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn value(self) -> u64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn part1_score(self, other: Move) -> u64 {
        other.value() + other.outcome(self).value()
    }

    fn part2_score(self, outcome: Outcome) -> u64 {
        outcome.value()
            + (match (self, outcome) {
                (_, Draw) => self,
                (Rock, Lose) => Scissors,
                (Rock, Win) => Paper,
                (Paper, Lose) => Rock,
                (Paper, Win) => Scissors,
                (Scissors, Lose) => Paper,
                (Scissors, Win) => Rock,
            })
            .value()
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut p1 = 0_u64;
    let mut p2 = 0_u64;
    include_str!("input.txt").lines().for_each(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let left = left.parse::<Move>().unwrap();
        p1 += left.part1_score(right.parse().unwrap());
        p2 += left.part2_score(right.parse().unwrap());
    });

    (p1, p2)
}
