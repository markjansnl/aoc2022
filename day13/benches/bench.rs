use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day13::{decoder_key, input, right_order_count};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day13");
    group.sample_size(50);

    group.bench_function("part1", |b| {
        b.iter(|| right_order_count(black_box(input::USER)))
    });

    group.bench_function("part2", |b| b.iter(|| decoder_key(black_box(input::USER))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
