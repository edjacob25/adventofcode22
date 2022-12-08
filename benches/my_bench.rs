use advent22::problems::*;
use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($i:ident, $day:literal, $day_path:ident) => {
        let mut day = $i.benchmark_group(concat!("Day ", $day));
        day.bench_function("1", |b| b.iter(|| $day_path::part1()));
        day.bench_function("2", |b| b.iter(|| $day_path::part2()));
        day.finish();
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_day!(c, "01", day1);
    bench_day!(c, "02", day2);
    bench_day!(c, "03", day3);
    bench_day!(c, "04", day4);
    bench_day!(c, "05", day5);
    bench_day!(c, "06", day6);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
