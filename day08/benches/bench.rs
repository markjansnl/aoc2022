use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day08::{input, Grid};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day08");

    group.bench_function("part1", |b| {
        b.iter(|| Grid::from(black_box(input::USER)).count_visible())
    });

    group.bench_function("part2", |b| {
        b.iter(|| Grid::from(black_box(input::USER)).max_scenic_score())
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
