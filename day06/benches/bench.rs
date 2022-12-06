use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day06::{first_marker, input};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day06");
    group.sample_size(60);
    group.bench_function("part1", |b| {
        b.iter(|| first_marker::<4>(black_box(input::USER)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| first_marker::<14>(black_box(input::USER)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
