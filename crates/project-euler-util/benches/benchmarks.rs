use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use project_euler_util::{fibonacci, Primes};

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 10", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(10)).collect::<Vec<_>>())
    });
    c.bench_function("fib 20", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(20)).collect::<Vec<_>>())
    });
    c.bench_function("fib 40", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(40)).collect::<Vec<_>>())
    });
    c.bench_function("fib 60", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(60)).collect::<Vec<_>>())
    });
}

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes");
    for n in [1000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter_with_large_drop(|| Primes::with_capacity(n).take(n).collect::<Vec<_>>())
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci, bench_primes);
criterion_main!(benches);
