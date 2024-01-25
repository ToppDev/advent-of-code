use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::*;

fn part1(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| part1::process(black_box(include_str!("../input1.txt"))))
    });
    c.bench_function("part1_iter", |b| {
        b.iter(|| part1::process_iterator(black_box(include_str!("../input1.txt"))))
    });
}

fn part2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        b.iter(|| part2::process(black_box(include_str!("../input2.txt"))))
    });
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
