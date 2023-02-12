use criterion::{black_box, criterion_group, Criterion};
use incr_stats::{batch, incr::Stats};
use rand::Rng;

const VALUES_10: [f64; 10] = [-1.0, 2.3, 5.4, 3.0, 2.3, 3.6, 9.2, -2.3, -23.0, 1.0];

pub fn iter_10_mean(c: &mut Criterion) {
    let mut d = Stats::new();
    c.bench_function("iter_10_mean", |b| {
        b.iter(|| {
            for v in &VALUES_10 {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.mean()).unwrap();
        })
    });
}

pub fn batch_10_mean(c: &mut Criterion) {
    c.bench_function("batch_10_mean", |b| {
        b.iter(|| batch::mean(black_box(&VALUES_10)).unwrap())
    });
}

pub fn iter_1000_mean(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    let mut d = Stats::new();
    c.bench_function("iter_1000_mean", |b| {
        b.iter(|| {
            for v in &a {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.mean()).unwrap();
        })
    });
}

pub fn batch_1000_mean(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    c.bench_function("batch_1000_mean", |b| {
        b.iter(|| batch::mean(black_box(&a)).unwrap())
    });
}

pub fn iter_10_sample_kurtosis(c: &mut Criterion) {
    let mut d = Stats::new();
    c.bench_function("iter_10_sample_kurtosis", |b| {
        b.iter(|| {
            for v in &VALUES_10 {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.sample_kurtosis()).unwrap();
        })
    });
}

pub fn batch_10_sample_kurtosis(c: &mut Criterion) {
    c.bench_function("batch_10_sample_kurtosis", |b| {
        b.iter(|| batch::sample_kurtosis(black_box(&VALUES_10)).unwrap())
    });
}

pub fn iter_1000_sample_kurtosis(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    let mut d = Stats::new();
    c.bench_function("iter_1000_sample_kurtosis", |b| {
        b.iter(|| {
            for v in &a {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.sample_kurtosis()).unwrap();
        })
    });
}

pub fn batch_1000_sample_kurtosis(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    c.bench_function("batch_1000_sample_kurtosis", |b| {
        b.iter(|| batch::sample_kurtosis(black_box(&a)).unwrap())
    });
}

criterion_group!(
    benches,
    iter_10_mean,
    batch_10_mean,
    iter_1000_mean,
    batch_1000_mean,
    batch_10_sample_kurtosis,
    iter_10_sample_kurtosis,
    iter_1000_sample_kurtosis,
    batch_1000_sample_kurtosis,
);
