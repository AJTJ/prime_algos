// #![feature(test)]
use prime_finder::eratosthenes;

const TEST_NUMBER_LARGE: usize = 1000000;
const TEST_NUMBER_SMALL: usize = 500;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }
// fn simple_time(c: &mut Criterion) {
//     c.bench_function("simple time", |b| b.iter(|| eratosthenes::simple_time()));
// }

pub fn eratosthenes_benchmark_large(c: &mut Criterion) {
    c.bench_function("Eratosthenes single large ", |b| {
        b.iter(|| eratosthenes::single_thread(black_box(TEST_NUMBER_LARGE)))
    });
}

pub fn eratosthenes_benchmark_small(c: &mut Criterion) {
    c.bench_function("Eratosthenes single small", |b| {
        b.iter(|| eratosthenes::single_thread(black_box(TEST_NUMBER_SMALL)))
    });
}

criterion_group!(
    benches,
    // criterion_benchmark,
    // simple_time,
    eratosthenes_benchmark_large,
    eratosthenes_benchmark_small
);
criterion_main!(benches);

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//     #[bench]
//     fn single_eras(b: &mut Bencher) {
//         let erat = eratosthenes::single_thread(100);
//     }
// }
