use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day22::{input, part1};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day22");
    // group.sample_size(10);

    group.bench_function("part1", |b| b.iter(|| part1(black_box(input::USER))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
