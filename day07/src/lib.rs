use std::fmt::Display;

use peeking_take_while::PeekableExt;

const MAX_TO_SUM: u64 = 100_000;
const DISK_SPACE: u64 = 70_000_000;
const NEEDED_SPACE: u64 = 30_000_000;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // ASSUMPTION: Directories are listed in DFS order, with no directory being ls-ed more than
    // once.
    let mut input = include_str!("input.txt").trim().lines().peekable();

    let mut weights: Vec<u64> = Vec::with_capacity(256);
    let mut stack: Vec<usize> = Vec::with_capacity(16);
    let mut next_id = 0;

    while let Some(line) = input.next() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            // We're changing directory
            if dir == ".." {
                // If we're going up, add the final weight of this directory to our parent.
                let prev = stack.pop().unwrap();
                let final_weight = weights[prev];
                weights[*stack.last().unwrap()] += final_weight;
            } else {
                // If we're going down, assign this directory an ID.
                stack.push(next_id);
                next_id += 1;
                weights.push(0);
            }
        } else {
            // line must be "$ ls"
            // For each file in the list, add its size to the current directory.
            // Ignoring directories, as those are handled when going up.
            let &cwd = stack.last().unwrap();
            input
                .by_ref()
                .peeking_take_while(|line| !line.starts_with('$'))
                .map(|line| line.split_once(' ').unwrap())
                .filter(|&(ty, _)| ty != "dir")
                .for_each(|(size, _)| weights[cwd] += size.parse::<u64>().unwrap());
        }
    }

    // After our traversal is over, add up what we didn't `cd` out of.
    loop {
        let prev = stack.pop().unwrap();
        if prev == 0 {
            break;
        }
        let final_weight = weights[prev];
        weights[*stack.last().unwrap()] += final_weight;
    }

    // Sum up all the small-enough directories.
    let p1: u64 = weights.iter().filter(|&&v| v <= MAX_TO_SUM).sum();

    // Get the smallest directory big enough to be worth deleting.
    let delete_target = NEEDED_SPACE - (DISK_SPACE - weights[0]);
    let p2 = weights
        .iter()
        .filter(|&&size| size >= delete_target)
        .min()
        .copied()
        .unwrap();

    (p1, p2)
}
