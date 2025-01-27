use criterion::{criterion_group, criterion_main, Criterion};
use project_euler::problems::solve;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("problem 0002", |b| b.iter_with_large_drop(|| solve(2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
