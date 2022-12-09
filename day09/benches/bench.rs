use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day09::{count_tail_positions, input};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day09");

    group.bench_function("part1", |b| {
        b.iter(|| count_tail_positions(black_box(input::USER), 2))
    });

    group.bench_function("part2", |b| {
        b.iter(|| count_tail_positions(black_box(input::USER), 10))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
