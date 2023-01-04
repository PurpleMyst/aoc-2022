use std::{
    collections::HashMap,
    fmt::Display,
    mem::swap,
    ops::{Add, Div, Mul, Sub},
};

/// At-most linear polynomial
#[derive(Debug, Clone, Copy)]
struct Polynomial(f64, f64);

impl Polynomial {
    fn is_number(&self) -> bool {
        self.1 == 0.0
    }

    #[track_caller]
    fn unwrap_number(self) -> f64 {
        debug_assert!(self.is_number());
        self.0
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        if !rhs.is_number() {
            swap(&mut self, &mut rhs);
        }
        let k = rhs.unwrap_number();
        Self(self.0 * k, self.1 * k)
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let k = rhs.unwrap_number();
        Self(self.0 / k, self.1 / k)
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Constant(Polynomial),
    Add(&'static str, &'static str),
    Sub(&'static str, &'static str),
    Mul(&'static str, &'static str),
    Div(&'static str, &'static str),
}

impl Operation {
    fn parse(s: &'static str) -> Self {
        if let Ok(n) = s.parse::<f64>() {
            return Self::Constant(Polynomial(n, 0.));
        }

        let mut parts = s.splitn(3, ' ');
        let lhs = parts.next().unwrap();
        let op = parts.next().unwrap();
        let rhs = parts.next().unwrap();

        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("unknown operation {op:?}"),
        };

        op(lhs, rhs)
    }

    fn evaluate(self, monkeys: &mut HashMap<&str, Self>) -> Polynomial {
        let mut get_and_eval = |name| monkeys.remove(name).unwrap().evaluate(monkeys);

        match self {
            Operation::Constant(n) => n,
            Operation::Add(lhs, rhs) => get_and_eval(lhs) + get_and_eval(rhs),
            Operation::Sub(lhs, rhs) => get_and_eval(lhs) - get_and_eval(rhs),
            Operation::Mul(lhs, rhs) => get_and_eval(lhs) * get_and_eval(rhs),
            Operation::Div(lhs, rhs) => get_and_eval(lhs) / get_and_eval(rhs),
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let monkeys = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            (lhs, Operation::parse(rhs))
        })
        .collect::<HashMap<_, _>>();

    let p1 = {
        let mut monkeys = monkeys.clone();
        monkeys
            .remove("root")
            .unwrap()
            .evaluate(&mut monkeys.clone())
            .unwrap_number()
    };

    let p2 = {
        let mut monkeys = monkeys;
        let (alice, bob) = match monkeys.remove("root").unwrap() {
            Operation::Constant(_) => todo!(),
            Operation::Add(alice, bob) => (alice, bob),
            Operation::Sub(alice, bob) => (alice, bob),
            Operation::Mul(alice, bob) => (alice, bob),
            Operation::Div(alice, bob) => (alice, bob),
        };
        monkeys.insert("humn", Operation::Constant(Polynomial(0., 1.)));
        let alice_val = monkeys.remove(alice).unwrap().evaluate(&mut monkeys);
        let bob_val = monkeys.remove(bob).unwrap().evaluate(&mut monkeys);
        (bob_val.0 - alice_val.0) / (alice_val.1 - bob_val.1)
    };

    (p1, p2)
}
