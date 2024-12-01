use {{crate_name}}::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2(c: &mut Criterion) {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}