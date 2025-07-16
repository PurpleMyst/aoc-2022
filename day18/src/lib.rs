use std::{
    fmt::Display,
};

use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut face_centers = HashMap::<_, usize>::default();

    let cubes = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i8>().unwrap() * 2);
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            [x, y, z]
        })
        .collect::<HashSet<_>>();

    cubes.iter().copied().for_each(|cube_center| {
        for axis in [0, 1, 2] {
            for delta in [-1, 1] {
                let mut face_center = cube_center;
                face_center[axis] += delta;
                *face_centers.entry(face_center).or_default() += 1;
            }
        }
    });

    face_centers.retain(|_, v| *v == 1);
    let part1 = face_centers.len();

    const PADDING: i8 = 2;

    let (x_min, x_max) = cubes.iter().map(|c| c[0]).minmax().into_option().unwrap();
    let (y_min, y_max) = cubes.iter().map(|c| c[1]).minmax().into_option().unwrap();
    let (z_min, z_max) = cubes.iter().map(|c| c[2]).minmax().into_option().unwrap();

    let in_bounds = |c: &[i8; 3]| {
        c[0] >= x_min - PADDING
            && c[0] <= x_max + PADDING
            && c[1] >= y_min - PADDING
            && c[1] <= y_max + PADDING
            && c[2] >= z_min - PADDING
            && c[2] <= z_max + PADDING
    };

    let mut q = vec![[x_max , y_max , z_max ]];
    let mut visited = HashSet::<[i8; 3]>::default();
    while let Some([x, y, z]) = q.pop() {
        let cube_center = [x, y, z];
        visited.insert(cube_center);

        for axis in [0, 1, 2] {
            for delta in [-2, 2] {
                let mut new_pos = cube_center;
                new_pos[axis] += delta;

                if in_bounds(&new_pos) && !visited.contains(&new_pos) && !cubes.contains(&new_pos) {
                    q.push(new_pos);
                }
            }
        }
    }

    let part2 = itertools::iproduct!(visited.into_iter(), [0, 1, 2], [-1, 1])
        .filter_map(|(c, axis, delta)| {
            let mut face_center = c;
            face_center[axis] += delta;
            face_centers.contains_key(&face_center).then_some(face_center)
        })
        .collect::<HashSet<_>>()
        .len();

    (part1, part2)
}
