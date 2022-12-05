use std::fmt::Display;

fn do_solve(mut stacks: Vec<Vec<char>>, instructions: &str, reverse: bool) -> String {
    let mut buf = Vec::new();
    for instruction in instructions.lines() {
        let mut parts = instruction
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| n.parse::<usize>().unwrap());
        let amount = parts.next().unwrap();
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let offset = stacks[from - 1].len() - amount;
        buf.clear();
        buf.extend(stacks[from - 1].drain(offset..));
        if reverse {
            buf.reverse()
        };
        stacks[to - 1].append(&mut buf);
    }
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect::<String>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (initial_state, instructions) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut stacks = vec![vec![]; (initial_state.find('\n').unwrap() + 3) / 4];

    initial_state.lines().for_each(|row| {
        if row.chars().nth(1).map_or(true, |ch| ch.is_numeric()) {
            return;
        }
        stacks
            .iter_mut()
            .zip(row.chars().skip(1).step_by(4))
            .for_each(|(stack, elem)| {
                if elem != ' ' {
                    stack.push(elem)
                }
            });
    });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    (
        do_solve(stacks.clone(), instructions, true),
        do_solve(stacks, instructions, false),
    )
}
