//        aoc_2022, benchmarks.rs, David Govorko, 2022-12-01, MIT Or Apache 2.0

use aoc_2022_lib::{
   day1::{day1_puzzle1_attempt1, day1_puzzle1_attempt2, day1_puzzle2_attempt1},
   day2::{day2_puzzle1_attempt1, day2_puzzle2_attempt1, day2_puzzle2_attempt2},
   day3::{day3_puzzle1_attempt1, day3_puzzle1_attempt2, day3_puzzle2_attempt1},
};
use criterion::{criterion_group, criterion_main, Criterion};

// Note, these benches are with outputs suppressed, assuming they yield the correct answer
// day 1 | puzzle 1        time:   [54.417 µs 55.092 µs 56.215 µs]
// day 1 | puzzle 1 | a2   time:   [62.407 µs 63.792 µs 65.349 µs]
// day 1 | puzzle 2        time:   [56.904 µs 57.597 µs 58.515 µs]
fn day_1_benchmark(c: &mut Criterion) {
   c.bench_function("day 1 | puzzle 1", |b| {
      b.iter(|| day1_puzzle1_attempt1("src/inputs/day 1/puzzle.txt"))
   });
   c.bench_function("day 1 | puzzle 1 | a2", |b| {
      b.iter(|| day1_puzzle1_attempt2("src/inputs/day 1/puzzle.txt"))
   });
   c.bench_function("day 1 | puzzle 2", |b| {
      b.iter(|| day1_puzzle2_attempt1("src/inputs/day 1/puzzle.txt"))
   });
}

// Note, these benches are with outputs suppressed, assuming they yield the correct answer
// day 2 | puzzle 1        time:   [61.394 µs 61.953 µs 62.662 µs]
// day 2 | puzzle 2        time:   [60.344 µs 60.923 µs 61.677 µs]
// day 2 | puzzle 2 | a2   time:   [59.760 µs 60.147 µs 60.569 µs]
fn day_2_benchmark(c: &mut Criterion) {
   c.bench_function("day 2 | puzzle 1", |b| {
      b.iter(|| day2_puzzle1_attempt1("src/inputs/day 2/puzzle.txt"))
   });
   c.bench_function("day 2 | puzzle 2", |b| {
      b.iter(|| day2_puzzle2_attempt1("src/inputs/day 2/puzzle.txt"))
   });
   c.bench_function("day 2 | puzzle 2 | a2", |b| {
      b.iter(|| day2_puzzle2_attempt2("src/inputs/day 2/puzzle.txt"))
   });
}

// Note, these benches are with outputs suppressed, assuming they yield the correct answer
// day 3 | puzzle 1        time:   [277.82 µs 280.66 µs 284.04 µs]
// day 3 | puzzle 1 | a2   time:   [50.096 µs 51.247 µs 52.639 µs]
// day 3 | puzzle 2        time:   [312.03 µs 313.62 µs 315.58 µs]
fn day_3_benchmark(c: &mut Criterion) {
   c.bench_function("day 3 | puzzle 1", |b| {
      b.iter(|| day3_puzzle1_attempt1("src/inputs/day 3/puzzle.txt"))
   });
   c.bench_function("day 3 | puzzle 1 | a2", |b| {
      b.iter(|| day3_puzzle1_attempt2("src/inputs/day 3/puzzle.txt"))
   });
   c.bench_function("day 3 | puzzle 2", |b| {
      b.iter(|| day3_puzzle2_attempt1("src/inputs/day 3/puzzle.txt"))
   });
}

criterion_group!(benches, day_1_benchmark, day_2_benchmark, day_3_benchmark);
criterion_main!(benches);
