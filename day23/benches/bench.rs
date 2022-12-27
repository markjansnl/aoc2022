use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day23::{input, part1, part2};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day23");
    group.sample_size(10);

    group.bench_function("part1", |b| b.iter(|| part1(black_box(input::USER))));

    group.bench_function("part2", |b| b.iter(|| part2(black_box(input::USER))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
