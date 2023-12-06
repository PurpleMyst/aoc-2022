use std::fmt::Display;

const DECRYPTION_KEY: isize = 811_589_153;

fn mix(numbers: &mut Vec<(usize, isize)>) {
    for id in 0..numbers.len() {
        let pos = numbers.iter().position(|(id2, _)| *id2 == id).unwrap();
        let (_, number) = numbers.remove(pos);
        let new_pos = usize::try_from((isize::try_from(pos).unwrap()  + number).rem_euclid(isize::try_from(numbers.len()).unwrap())).unwrap();
        numbers.insert(new_pos, (id, number));
    }
}

fn extract_answer(numbers: &[(usize, isize)]) -> isize {
    let zero_pos = numbers.iter().position(|(_, n)| *n == 0).unwrap();
    let offsets = [1000, 2000, 3000];
    offsets
        .into_iter()
        .map(|offset| (zero_pos + offset) % numbers.len())
        .map(|pos| numbers[pos].1)
        .sum::<isize>()
}

fn part1(mut numbers: Vec<(usize, isize)>) -> isize {
    mix(&mut numbers);
    extract_answer(&numbers)
}

fn part2(mut numbers: Vec<(usize, isize)>) -> isize {
    numbers.iter_mut().for_each(|(_, n)| *n *= DECRYPTION_KEY);
    for _ in 0..10 { mix(&mut numbers) };
    extract_answer(&numbers)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let numbers = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();

    (part1(numbers.clone()), part2(numbers))
}
