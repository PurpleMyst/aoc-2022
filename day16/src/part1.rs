use super::*;

#[derive(Debug)]
pub struct State {
    opened: Opened,
    total: u16,
    per_second: u16,
    time_left: u8,
    position: NodeIndex,
}

impl ProblemState for State {
    fn initial(position: NodeIndex) -> Self {
        Self {
            opened: Opened::default(),
            position,
            total: 0,
            per_second: 0,
            time_left: 30,
        }
    }

    fn advance(self, graph: &UnGraph<u16, u8>, states: &mut Vec<Self>) {
        let state = self;
        if !state.opened.contains(state.position) {
            states.push(Self {
                opened: state.opened.update(state.position),
                position: state.position,
                total: state.total + state.per_second,
                per_second: state.per_second + *graph.node_weight(state.position).unwrap(),
                time_left: state.time_left - 1,
            });
        }

        states.extend(graph.edges(state.position).filter_map(|edge| {
            let &distance = edge.weight();
            let next = edge.target();
            Some(Self {
                time_left: state.time_left.checked_sub(distance)?,
                opened: state.opened,
                position: next,
                total: state.total + state.per_second * u16::from(distance),
                per_second: state.per_second,
            })
        }))
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
