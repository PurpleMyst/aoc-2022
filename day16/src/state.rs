use std::fmt::Debug;

use super::*;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Opened(u16);

impl Opened {
    pub fn update(self, idx: NodeIndex) -> Self {
        Self(self.0 | (1 << idx.index()))
    }

    pub fn contains(self, idx: NodeIndex) -> bool {
        (self.0 & (1 << idx.index())) != 0
    }
}

impl Debug for Opened {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Opened").field(&format_args!("{:016b}", self.0)).finish()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct State<const PART2: bool> {
    opened: Opened,
    relieved: u16,
    hypothetical_flow: u16,
    time_left: u8,
    position: NodeIndex,
    can_rewind: bool,
}

impl<const PART2: bool> State<PART2> {
    const SECONDS: u8 = if PART2 { 26 } else { 30 };

    pub fn new(hypothetical_flow: u16) -> Self {
        Self {
            opened: Opened::default(),
            position: NodeIndex::new(0),
            relieved: 0,
            hypothetical_flow,
            time_left: Self::SECONDS,
            can_rewind: PART2,
        }
    }

    pub fn upper_bound(&self) -> u16 {
        if self.can_rewind {
            self.relieved + self.hypothetical_flow * u16::from(Self::SECONDS)
        } else {
            self.relieved + self.hypothetical_flow * u16::from(self.time_left)
        }
    }

    pub fn advance<'a>(self, flows: &'a [u16; 16], distances: &'a [u8; 256]) -> impl Iterator<Item = Self> + 'a {
        let hops = (0..16)
            .map(NodeIndex::new)
            .filter(move |&idx| !self.opened.contains(idx))
            .filter_map(move |next| {
                let flow = flows[next.index()];
                let distance = distances[self.position.index() * 16 + next.index()];
                let new_time_left = self.time_left.checked_sub(distance + 1)?;
                Some(Self {
                    time_left: new_time_left,
                    relieved: self.relieved + u16::from(new_time_left) * flow,
                    opened: self.opened.update(next),
                    position: next,
                    hypothetical_flow: self.hypothetical_flow - flow,
                    ..self
                })
            });

        let rewind = if self.can_rewind {
            Some(Self {
                position: NodeIndex::new(0),
                time_left: Self::SECONDS,
                can_rewind: false,
                ..self
            })
        } else {
            None
        };

        hops.chain(rewind)
    }

    pub fn relieved(&self) -> u16 {
        self.relieved
    }
}
