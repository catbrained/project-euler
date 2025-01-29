use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project_euler_util::fibonacci;

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 10", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(20)).collect::<Vec<_>>())
    });
    c.bench_function("fib 20", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(20)).collect::<Vec<_>>())
    });
    c.bench_function("fib 40", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(20)).collect::<Vec<_>>())
    });
    c.bench_function("fib 60", |b| {
        b.iter_with_large_drop(|| fibonacci().take(black_box(20)).collect::<Vec<_>>())
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
