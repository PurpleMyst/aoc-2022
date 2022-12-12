use std::{env, fs};

fn main() {
    let (part1, part2) = day12::do_solve(&fs::read_to_string(env::args().nth(1).unwrap()).unwrap());
    println!("{part1}");
    println!("{part2}");
}
