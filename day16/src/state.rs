use std::fmt::Debug;

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Opened(u16);

impl Opened {
    pub fn update(self, idx: usize) -> Self {
        Self(self.0 | (1 << idx))
    }

    pub fn contains(self, idx: usize) -> bool {
        (self.0 & (1 << idx)) != 0
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
    position: u8,
    can_rewind: bool,
}

impl<const PART2: bool> State<PART2> {
    pub const START: u8 = 0;
    const SECONDS: u8 = if PART2 { 26 } else { 30 };

    pub fn new(hypothetical_flow: u16) -> Self {
        Self {
            opened: Opened::default(),
            position: Self::START,
            relieved: 0,
            hypothetical_flow,
            time_left: Self::SECONDS,
            can_rewind: PART2,
        }
    }

    pub fn relieved(&self) -> u16 {
        self.relieved
    }

    pub fn upper_bound(&self) -> u16 {
        if self.can_rewind {
            self.relieved + self.hypothetical_flow * u16::from(Self::SECONDS)
        } else {
            self.relieved + self.hypothetical_flow * u16::from(self.time_left)
        }
    }

    pub fn advance(self, flows: &[u16; 16], distances: &[u8; 256], states: &mut impl Extend<Self>) {
        // Try to open each valve that isn't currently open.
        states.extend(
            flows
                .iter()
                .copied()
                .enumerate()
                .filter(move |&(valve, _)| !self.opened.contains(valve))
                .filter_map(move |(valve, flow)| {
                    let distance = distances[self.position as usize * 16 + valve];
                    let new_time_left = self.time_left.checked_sub(distance + 1)?;
                    Some(Self {
                        time_left: new_time_left,
                        relieved: self.relieved + u16::from(new_time_left) * flow,
                        opened: self.opened.update(valve),
                        position: valve as u8,
                        hypothetical_flow: self.hypothetical_flow - flow,
                        ..self
                    })
                }),
        );

        // Try to rewind if we can do so.
        states.extend(self.can_rewind.then_some(Self {
            position: Self::START,
            time_left: Self::SECONDS,
            can_rewind: false,
            ..self
        }));
    }
}
