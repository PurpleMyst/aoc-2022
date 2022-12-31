use std::{cmp, fmt::Display, str::Lines};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Increment(u64),
    Scale(u64),
    Square,
}

impl Operation {
    fn parse(s: &str) -> Self {
        if s == "new = old * old" {
            return Self::Square;
        }

        let mut parts = s.split(' ').skip(3);
        let op = parts.next().unwrap();
        let rhs = parts.next().unwrap().parse().unwrap();
        match op {
            "+" => Self::Increment(rhs),
            "*" => Self::Scale(rhs),
            _ => unreachable!(),
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Increment(m) => old + m,
            Operation::Scale(m) => old * m,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
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
        let test = lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap();
        let targets = (
            lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap(),
            lines.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap(),
        );
        Self {
            items,
            operation,
            test,
            targets,
            inspected: 0,
        }
    }

    fn turn(&mut self, divide_by_three: bool, lcm: u64, buf: &mut [Vec<u64>]) {
        self.inspected += self.items.len();

        for mut item in self.items.drain(..) {
            item = self.operation.apply(item) % lcm;
            if divide_by_three {
                item /= 3;
            }
            let target = if item % self.test == 0 {
                self.targets.0
            } else {
                self.targets.1
            };
            buf[target].push(item);
        }
    }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, divide_by_three: bool) -> usize {
    let lcm = monkeys.iter().fold(1, |acc, monkey| num_integer::lcm(acc, monkey.test));

    let mut buf = vec![Vec::with_capacity(32); monkeys.len()];

    for _ in 0..rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.append(&mut buf[idx]);
            monkey.turn(divide_by_three, lcm, &mut buf);
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

    (simulate(monkeys.clone(), 20, true), simulate(monkeys, 10_000, false))
}
