//        aoc_2022, benchmarks.rs, David Govorko, 2022-12-01, MIT Or Apache 2.0

use aoc_2022_lib::{
   day1::{day1_puzzle1_attempt1, day1_puzzle1_attempt2, day1_puzzle2_attempt1},
   day2::{day2_puzzle1_attempt1, day2_puzzle2_attempt1, day2_puzzle2_attempt2},
};
use criterion::{criterion_group, criterion_main, Criterion};

// Note, these benches are with outputs suppressed, assuming they yield the correct answer
// day 1 | puzzle 1        time:   [54.535 µs 55.048 µs 55.721 µs]
// day 1 | puzzle 1 | a2   time:   [57.716 µs 58.380 µs 59.452 µs]
// day 1 | puzzle 2        time:   [57.625 µs 57.907 µs 58.291 µs]
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
// day 2 | puzzle 1        time:   [67.349 µs 67.831 µs 68.409 µs]
// day 2 | puzzle 2        time:   [67.516 µs 68.046 µs 68.698 µs]
// day 2 | puzzle 2 | a2   time:   [65.178 µs 65.428 µs 65.768 µs]
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

criterion_group!(benches, day_1_benchmark, day_2_benchmark);
criterion_main!(benches);
