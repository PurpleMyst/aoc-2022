#![feature(int_roundings)]

use std::{fmt::Display, iter::once, time::Instant};

use rayon::prelude::*;
use derive_more::{Deref, DerefMut};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug, Clone, Copy, Default, Deref, DerefMut)]
struct RobotCost([u8; 3]);

#[derive(Debug, Clone, Copy, Default, Deref, DerefMut)]
struct Blueprint { #[deref] #[deref_mut] robot_costs: [RobotCost; 4], max_cost_per_resource: [u8; 3] }

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
                "ore" | "ore." => &mut this[ORE],
                "clay" | "clay." => &mut this[CLAY],
                "obsidian" | "obsidian." => &mut this[OBSIDIAN],
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
        let robot_costs = [
            costs.next().unwrap(),
            costs.next().unwrap(),
            costs.next().unwrap(),
            costs.next().unwrap(),
        ] ;
        let max_per_resource = [ORE, CLAY, OBSIDIAN]
            .map(|idx| robot_costs.iter().map(|rc| rc[idx]).max().unwrap());
        Self { robot_costs, max_cost_per_resource: max_per_resource }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct State {
    time_left: u16,
    amounts: [u16; 4],
    robots: [u16; 4],
}

impl State {
    /// How many geodes have we gotten so far?
    fn geodes(&self) -> u16 {
        self.amounts[GEODE]
    }

    /// How many geodes can we get at most from this state?
    fn upper_bound(&self) -> u16 {
        self.amounts[GEODE] + self.robots[GEODE] * self.time_left + (1..self.time_left).sum::<u16>()
    }

    /// Make time pass and generate resources.
    fn advance_time(&mut self, t: u16) {
        self.amounts
            .iter_mut()
            .zip(self.robots.iter())
            .for_each(|(amount, robots)| *amount += robots * t);
        self.time_left -= t;
    }

    fn time_to_build(&self, costs: &RobotCost) -> Option<u16> {
        let mut result = 0;
        for ((have, robots), cost) in self
            .amounts
            .into_iter()
            .zip(self.robots.into_iter())
            .zip(costs.into_iter().map(u16::from))
        {
            let need = cost.saturating_sub(have);
            if robots == 0 {
                if need == 0 {
                    continue;
                } else {
                    return None;
                }
            }
            result = result.max(need.div_ceil(robots));
        }
        Some(result)
    }

    /// Build a robot, subtracting its cost.
    fn build(&mut self, idx: usize, cost: &RobotCost) {
        self.robots[idx] += 1;
        self.amounts.iter_mut().zip(cost.into_iter()).for_each(|(amount, c)| {
            *amount -= u16::from(c);
        });
    }

    /// Can we always build one geode robot from this point forward?
    fn can_always_build(&self, cost: &RobotCost) -> bool {
        self.robots
            .into_iter()
            .zip(cost.into_iter().map(u16::from))
            .all(|(r, c)| c <= r)
    }

    fn try_build(&self, idx: usize, cost: &RobotCost) -> Option<Self> {
        let t = self.time_to_build(cost)?;
        if t + 1 > self.time_left { return None;  }
        let mut next = self.clone();
        next.advance_time(t + 1);
        next.build(idx, cost);
        Some(next)
    }

    /// Add to the states structure the possible next states.
    fn branch(&self, blueprint: &Blueprint, states: &mut impl Extend<State>) {
        // Oh no, we've run out of time!
        if self.time_left == 0 {
            return;
        }

        // If we can always build geode robots from this point on, then we can shortcurt this state:
        // That's what's most convenient to do from now on.
        if self.can_always_build(&blueprint[GEODE]) {
            dbg!();
            let mut next = self.clone();
            next.time_left = 0;
            next.amounts[GEODE] = self.upper_bound();
            states.extend(once(next));
            return;
        }

        // For each resource: Do we make enough of that resource to build any robot we want? If not,
        // we'll need more robots!
        for resource in [ORE, CLAY, OBSIDIAN] {
            if self.robots[resource] < u16::from(blueprint.max_cost_per_resource[resource]) {
                states.extend(self.try_build(resource, &blueprint[resource]))
            }
        }

        // Build a geode robot! They're always useful.
        states.extend(self.try_build(GEODE, &blueprint[GEODE]));
    }
}

fn maximum_geodes(blueprint: Blueprint, time_alloted: u16) -> u16 {
    let mut states = Vec::new();
    let mut lower_bound = 0;

    let mut initial_state = State::default();
    initial_state.time_left = time_alloted;
    initial_state.robots[ORE] += 1;
    states.push(initial_state);

    while let Some(state) = states.pop() {
        if state.upper_bound() < lower_bound {
            continue;
        }

        let old_len = states.len();
        state.branch(&blueprint, &mut states);
        if states.len() == old_len && state.geodes() > lower_bound {
            lower_bound = state.geodes();
        }
    }
    lower_bound
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let blueprints: Vec<_> = (1..)
        .zip(include_str!("input.txt").lines().map(Blueprint::parse))
        .collect();

    let p1_start = Instant::now();
    let p1 = blueprints
        .clone()
        .into_par_iter()
        .map(|(id, blueprint)| {
            let start = Instant::now();
            let g = maximum_geodes(blueprint, 24);
            eprintln!(
                "Blueprint {id:2} gets a maximum of {g:2} geodes and takes {:.2?}",
                start.elapsed()
            );
            id * g
        })
        .sum::<u16>();
    eprintln!("Done with part 1 in {:.2?}", p1_start.elapsed());

    let p2_start = Instant::now();
    let p2 = blueprints[..3]
        .into_par_iter()
        .map(|&(id, blueprint)| {
            let start = Instant::now();
            let g = maximum_geodes(blueprint, 32);
            eprintln!(
                "Blueprint {id:2} gets a maximum of {g:2} geodes and takes {:.2?}",
                start.elapsed()
            );
            g
        })
        .product::<u16>();
    eprintln!("Done with part 2 in {:.2?}", p2_start.elapsed());

    (p1, p2)
}
