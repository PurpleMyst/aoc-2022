#![allow(dead_code)]

use super::*;

#[derive(Debug)]
pub struct State {
    opened: Opened,
    total: u16,
    per_second: u16,
    time_left: u8,
    position1: NodeIndex,
    position2: NodeIndex,
}

impl ProblemState for State {
    fn initial(start: NodeIndex) -> Self {
        Self {
            opened: Opened::default(),
            position1: start,
            position2: start,
            total: 0,
            per_second: 0,
            time_left: 26,
        }
    }

    fn advance(self, _graph: &UnGraph<u16, u8>, _states: &mut Vec<Self>) {
        todo!();
    }

    fn total(&self) -> u16 {
        self.total
    }

    fn time_left(&self) -> u8 {
        self.time_left
    }

    fn per_second(&self) -> u16 {
        self.per_second
    }

    fn opened(&self) -> Opened {
        self.opened
    }
}
