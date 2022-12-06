use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day05::{input, top_crates, CrateMover9000, CrateMover9001};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day05");
    // group.significance_level(0.1).sample_size(1000);
    group.bench_function("part1", |b| {
        b.iter(|| top_crates::<CrateMover9000>(black_box(input::USER)))
    });
    group.bench_function("part2", |b| {
        b.iter(|| top_crates::<CrateMover9001>(black_box(input::USER)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
