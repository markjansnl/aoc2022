use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day18::{input, surface_area};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day18");
    // group.sample_size(10);

    group.bench_function("part1", |b| {
        b.iter(|| surface_area(black_box(input::USER), false))
    });

    group.bench_function("part2", |b| {
        b.iter(|| surface_area(black_box(input::USER), true))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
