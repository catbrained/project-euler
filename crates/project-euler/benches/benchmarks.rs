use criterion::{criterion_group, criterion_main, Criterion};
use project_euler::problems::solve;

fn bench_p0002(c: &mut Criterion) {
    c.bench_function("problem 0002", |b| b.iter_with_large_drop(|| solve(2)));
}

fn bench_p0004(c: &mut Criterion) {
    c.bench_function("problem 0004", |b| b.iter_with_large_drop(|| solve(4)));
}

criterion_group!(benches, bench_p0002, bench_p0004);
criterion_main!(benches);
