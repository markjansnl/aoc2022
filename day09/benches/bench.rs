use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day09::{input, count_tail_positions, count_10th_tail_positions};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day09");

    group.bench_function("part1", |b| {
        b.iter(|| count_tail_positions(black_box(input::USER)))
    });

    group.bench_function("part2", |b| {
        b.iter(|| count_10th_tail_positions(black_box(input::USER)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
