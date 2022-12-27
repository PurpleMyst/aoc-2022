use std::{collections::HashMap, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut faces = HashMap::<_, usize>::new();

    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i8>().unwrap() * 2);
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            [x, y, z]
        })
        .for_each(|cube| {
            for axis in [0, 1, 2] {
                for delta in [-1, 1] {
                    let mut face = cube;
                    face[axis] += delta;
                    *faces.entry(face).or_default() += 1;
                }
            }
        });

    (faces.values().filter(|&&v| v == 1).count(), "TODO")
}
