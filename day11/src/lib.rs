use std::{cmp, collections::VecDeque, fmt::Display, str::Lines};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    IncrementOld(u64),
    ScaleOld(u64),
    SquareOld,
}

impl Operation {
    fn parse(s: &str) -> Self {
        if s == "new = old * old" {
            return Self::SquareOld;
        }

        let mut parts = s.split(' ').skip(3);
        let op = parts.next().unwrap();
        let rhs = parts.next().unwrap().parse().unwrap();
        match op {
            "+" => Self::IncrementOld(rhs),
            "*" => Self::ScaleOld(rhs),
            _ => unreachable!(),
        }
    }

    fn new(&self, old: u64) -> u64 {
        match self {
            Operation::IncrementOld(m) => old + m,
            Operation::ScaleOld(m) => old * m,
            Operation::SquareOld => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    targets: (usize, usize),
    inspected: usize,
}

impl Monkey {
    fn parse(lines: &mut Lines) -> Self {
        lines.next().unwrap();
        let items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let operation = Operation::parse(lines.next().unwrap().split_once(": ").unwrap().1);
        let test = lines
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let targets = (
            lines
                .next()
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .unwrap(),
            lines
                .next()
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .unwrap(),
        );
        Self {
            items,
            operation,
            test,
            targets,
            inspected: 0,
        }
    }

    fn inspect_one(&mut self, divide_by_three: bool, lcm: u64) -> Option<(usize, u64)> {
        let mut item = self.operation.new(self.items.pop_front()?) % lcm;
        if divide_by_three {
            item /= 3;
        }
        self.inspected += 1;
        let target = if item % self.test == 0 {
            self.targets.0
        } else {
            self.targets.1
        };
        Some((target, item))
    }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, divide_by_three: bool) -> usize {
    let lcm = monkeys
        .iter()
        .fold(1, |acc, monkey| num_integer::lcm(acc, monkey.test));

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some((target, value)) = monkeys[idx].inspect_one(divide_by_three, lcm) {
                monkeys[target].items.push_back(value);
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| cmp::Reverse(monkey.inspected))
        .k_smallest(2)
        .map(|cmp::Reverse(n)| n)
        .product::<usize>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut lines = include_str!("input.txt").trim().lines();
    let mut monkeys = Vec::with_capacity(8);
    loop {
        monkeys.push(Monkey::parse(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }

    (
        simulate(monkeys.clone(), 20, true),
        simulate(monkeys, 10_000, false),
    )
}
