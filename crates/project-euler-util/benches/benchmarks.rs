use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use project_euler_util::{fibonacci, Primes};

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for n in [10, 20, 40, 60].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter_with_large_drop(|| {
                let mut res = Vec::with_capacity(n);
                res.extend(fibonacci().take(n));
                res
            })
        });
    }
}

fn bench_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("primes");
    for n in [1000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter_with_large_drop(|| {
                let mut res = Vec::with_capacity(n);
                res.extend(Primes::with_capacity(n).take(n));
                res
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci, bench_primes);
criterion_main!(benches);
