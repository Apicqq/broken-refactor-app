use broken_app::{algo, normalize};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib_32", |b| b.iter(|| algo::slow_fib(black_box(32))));
}

fn bench_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("dedup_10k", |b| {
        b.iter(|| black_box(algo::slow_dedup(black_box(&data))))
    });
}

fn bench_normalize(c: &mut Criterion) {
    let input = " Hello World ".repeat(5_000);
    c.bench_function("normalize_65k", |b| {
        b.iter(|| black_box(normalize(black_box(&input))))
    });
}

criterion_group!(benches, bench_fib, bench_dedup, bench_normalize);
criterion_main!(benches);
