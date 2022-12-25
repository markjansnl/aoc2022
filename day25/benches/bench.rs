use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day25::{input, sum};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day25");
    // group.sample_size(10);

    group.bench_function("part1", |b| b.iter(|| sum(black_box(input::USER))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
