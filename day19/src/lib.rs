#![feature(int_roundings)]

use std::{fmt::Display, iter::once, time::Instant};

use binary_heap_plus::BinaryHeap;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
struct RobotCost([u8; 3]);

#[derive(Debug, Clone, Copy, Default)]
struct Blueprint([RobotCost; 4]);

impl RobotCost {
    fn parse(s: &str) -> Self {
        let mut this = Self::default();
        let (r1, r2) = s.split_once(" and ").unwrap_or((s, ""));
        for r in [r1, r2] {
            if r.is_empty() {
                continue;
            }
            let (n, ty) = r.split_once(' ').unwrap();
            let n: u8 = n.parse().unwrap();
            let c = match ty {
                "ore" | "ore." => &mut this.0[0],
                "clay" | "clay." => &mut this.0[1],
                "obsidian" | "obsidian." => &mut this.0[2],
                _ => unreachable!(),
            };
            *c += n;
        }
        this
    }
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let mut costs = s
            .split_once(": ")
            .unwrap()
            .1
            .split(". ")
            .map(|s| s.split_once(" costs ").unwrap().1)
            .map(RobotCost::parse);
        Self([
            costs.next().unwrap(),
            costs.next().unwrap(),
            costs.next().unwrap(),
            costs.next().unwrap(),
        ])
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Resource {
    robots: usize,
    amount: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct State {
    time_left: usize,
    resources: [Resource; 4],
}

impl State {
    fn total(&self) -> usize {
        self.resources[3].amount
    }

    fn priority(&self) -> impl Ord {
        self.total()
    }

    fn upper_bound(&self) -> usize {
        self.resources[3].amount + (0..self.time_left).map(|t| self.resources[3].robots + t).sum::<usize>()
    }

    fn tick(&mut self, n: usize) {
        self.resources.iter_mut().for_each(|r| {
            r.amount += r.robots * n;
        });
        self.time_left -= n;
    }

    fn advance_micro(&self, blueprint: &Blueprint, states: &mut impl Extend<State>) {
        if self.time_left == 0 {
            return;
        }
        states.extend(blueprint.0.iter().enumerate().filter_map(|(idx, cost)| {
            let can_build = self.resources.iter().zip(cost.0.into_iter()).all(|(r, c)| {
                let c = usize::from(c);
                c <= r.amount
            });
            if !can_build {
                return None;
            }

            let mut next = self.clone();
            next.tick(1);
            next.resources[idx].robots += 1;
            next.resources.iter_mut().zip(cost.0.into_iter()).for_each(|(r, c)| {
                r.amount -= usize::from(c);
            });

            Some(next)
        }));

        let mut do_nothing = self.clone();
        do_nothing.tick(1);
        states.extend(once(do_nothing));
    }
}

fn geodes(blueprint: Blueprint, time: usize) -> usize {
    let mut states = BinaryHeap::new_by_key(|state: &State| state.priority());
    let mut lower_bound = 0;

    let mut initial_state = State::default();
    initial_state.time_left = time;
    initial_state.resources[0].robots += 1;
    states.push(initial_state);

    while let Some(state) = states.pop() {
        if state.upper_bound() < lower_bound {
            continue;
        }

        let old_len = states.len();
        state.advance_micro(&blueprint, &mut states);
        if states.len() == old_len && state.total() > lower_bound {
            assert_eq!(state.time_left, 0);
            lower_bound = state.total();
        }
    }
    lower_bound
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let blueprints: Vec<_> = (1usize..)
        .zip(include_str!("input.txt").lines().map(Blueprint::parse))
        .collect();

    let p1 = blueprints
        .clone()
        .into_par_iter()
        .map(|(id, blueprint)| {
            let start = Instant::now();
            let g = geodes(blueprint, 24);
            println!("Blueprint {id:2} gets a maximum of {g:3} geodes and takes {:?}", start.elapsed());
            id * g
        })
        .sum::<usize>();

    let p2 = blueprints
        [..3]
        .into_par_iter()
        .map(|&(id, blueprint)| {
            let start = Instant::now();
            let g = geodes(blueprint, 32);
            println!("Blueprint {id:2} gets a maximum of {g:3} geodes and takes {:?}", start.elapsed());
            g
        })
        .product::<usize>();

    (p1, p2)
}
