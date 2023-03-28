// #![feature(test)]
// use prime_finder::eratosthenes;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
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
