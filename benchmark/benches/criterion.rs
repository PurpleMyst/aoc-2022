use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! doit {
    ($($day:ident),+$(,)?) => {
        pub fn aoc_benchmark(c: &mut Criterion) {
            $(c.bench_function(stringify!($day), |b| b.iter($day::solve));)+
            c.bench_function("all", |b| b.iter(|| ($($day::solve()),+)));
        }

        criterion_group! {
            name = benches;

            config = Criterion::default()
                .significance_level(0.1)
                .sample_size(500)
                .measurement_time(Duration::from_secs(15))
                .warm_up_time(Duration::from_secs(5))
                .noise_threshold(0.05);

            targets = aoc_benchmark
        }

        criterion_main!(benches);
    };
}

#[rustfmt::skip]
doit!(
    day01, day02, day03, day04, day05, day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day16,
    day15,
    day17,
    day18,
    day21,
    day23,
    day25,
    day20,
    day24,
    day22,
);
