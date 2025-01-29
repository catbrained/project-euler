use criterion::{criterion_group, criterion_main, Criterion};
use project_euler_util::fibonacci;

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| {
        b.iter_with_large_drop(|| fibonacci(4_000_000))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
