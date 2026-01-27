use criterion::{Criterion, criterion_group, criterion_main};
use datastory::analyze::{mean, median, mode, pearson_correlation, quartiles};

fn bench_mean(c: &mut Criterion) {
    let data: Vec<f64> = (0..10_000).map(|x| x as f64).collect();
    c.bench_function("mean", |b| b.iter(|| mean(&data)));
}

fn bench_median(c: &mut Criterion) {
    let data: Vec<f64> = (0..10_000).map(|x| x as f64).collect();
    c.bench_function("median", |b| b.iter(|| median(&data)));
}

fn bench_mode(c: &mut Criterion) {
    let data: Vec<String> = (0..10_000).map(|x| format!("val{}", x % 100)).collect();
    c.bench_function("mode", |b| b.iter(|| mode(&data)));
}

fn bench_quartiles(c: &mut Criterion) {
    let data: Vec<f64> = (0..10_000).map(|x| x as f64).collect();
    c.bench_function("quartiles", |b| b.iter(|| quartiles(&data)));
}

fn bench_pearson(c: &mut Criterion) {
    let x: Vec<f64> = (0..10_000).map(|x| x as f64).collect();
    let y: Vec<f64> = (0..10_000).map(|x| (x * 2) as f64).collect();
    c.bench_function("pearson_correlation", |b| {
        b.iter(|| pearson_correlation(&x, &y))
    });
}

criterion_group!(
    benches,
    bench_mean,
    bench_median,
    bench_mode,
    bench_quartiles,
    bench_pearson
);
criterion_main!(benches);
