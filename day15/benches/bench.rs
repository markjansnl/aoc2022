use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15::{input, nr_of_no_beacons_on_line, tuning_frequency};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day15");
    group.sample_size(10);

    group.bench_function("part1", |b| {
        b.iter(|| nr_of_no_beacons_on_line(2000000, black_box(input::USER)))
    });

    group.bench_function("part2", |b| b.iter(|| tuning_frequency(4000000, black_box(input::USER))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
