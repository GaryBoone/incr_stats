use criterion::{black_box, criterion_group, Criterion};
use incr_stats::{batch, incr, vec};
use rand::Rng;

const VALUES_10: [f64; 10] = [-1.0, 2.3, 5.4, 3.0, 2.3, 3.6, 9.2, -2.3, -23.0, 1.0];

pub fn mean_10(c: &mut Criterion) {
    c.bench_function("mean_10_incr", |b| {
        b.iter(|| {
            let mut d = incr::Stats::new();
            for v in &VALUES_10 {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.mean()).unwrap();
        })
    });

    c.bench_function("mean_10_batch", |b| {
        b.iter(|| batch::mean(black_box(&VALUES_10)).unwrap())
    });

    c.bench_function("mean_10_vec", |b| {
        let mut d = vec::Stats::new(&VALUES_10).unwrap();
        b.iter(|| d.mean().unwrap())
    });
}

pub fn mean_1000(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    c.bench_function("mean_1000_incr", |b| {
        b.iter(|| {
            let mut d = incr::Stats::new();
            for v in &a {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.mean()).unwrap();
        })
    });

    c.bench_function("1000_mean_batch", |b| {
        b.iter(|| batch::mean(black_box(&a)).unwrap())
    });

    c.bench_function("mean_1000_vec", |b| {
        let mut d = vec::Stats::new(&a).unwrap();
        b.iter(|| d.mean().unwrap())
    });
}

pub fn sample_kurtosis_10(c: &mut Criterion) {
    c.bench_function("sample_kurtosis_10_incr", |b| {
        b.iter(|| {
            let mut d = incr::Stats::new();
            for v in &VALUES_10 {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.sample_kurtosis()).unwrap();
        })
    });

    c.bench_function("sample_kurtosis_10_batch", |b| {
        b.iter(|| batch::sample_kurtosis(black_box(&VALUES_10)).unwrap())
    });

    c.bench_function("sample_kurtosis_10_vec", |b| {
        let mut d = vec::Stats::new(black_box(&VALUES_10)).unwrap();
        b.iter(|| d.sample_kurtosis().unwrap())
    });
}

pub fn sample_kurtosis_1000(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    c.bench_function("sample_kurtosis_1000_incr", |b| {
        b.iter(|| {
            let mut d = incr::Stats::new();
            for v in &a {
                d.update(black_box(*v)).unwrap()
            }
            black_box(d.sample_kurtosis()).unwrap();
        })
    });

    c.bench_function("sample_kurtosis_1000_batch", |b| {
        b.iter(|| batch::sample_kurtosis(black_box(&a)).unwrap())
    });

    c.bench_function("sample_kurtosis_1000_vec", |b| {
        let mut d = vec::Stats::new(black_box(&a)).unwrap();
        b.iter(|| d.sample_kurtosis().unwrap())
    });
}

criterion_group!(
    benches,
    mean_10,
    mean_1000,
    sample_kurtosis_10,
    sample_kurtosis_1000,
);
