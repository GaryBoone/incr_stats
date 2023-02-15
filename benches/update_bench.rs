use criterion::{black_box, criterion_group, Criterion};
use incr_stats::incr;
use rand::Rng;

const MAX_DATA_SIZE: u32 = 1_000_000;

macro_rules! update {
    ($c:expr, $count:expr) => {
        let mut rng = rand::thread_rng();
        let mut a = vec![];
        for _ in 0..$count {
            a.push(rng.gen())
        }
        let mut d = incr::Stats::new();
        let mut i = 0;
        $c.bench_function(&format!("update_{}_incr", $count), |b| {
            b.iter(|| {
                black_box(d.update(black_box(a[i])).unwrap());
                i = (i + 1) % a.len();
            })
        });
    };
}

macro_rules! final_calc {
    ($c:expr, $count:expr) => {
        let mut rng = rand::thread_rng();
        let mut d = incr::Stats::new();
        for _ in 0..$count {
            d.update(rng.gen()).unwrap()
        }
        $c.bench_function(&format!("final_calc_{}_incr", $count), |b| {
            b.iter(|| {
                black_box(d.sample_kurtosis().unwrap());
            })
        });
    };
}

pub fn update_10(c: &mut Criterion) {
    update!(c, 10);
}

pub fn update_1mm(c: &mut Criterion) {
    update!(c, MAX_DATA_SIZE);
}

pub fn final_calc_10(c: &mut Criterion) {
    final_calc!(c, 10);
}

pub fn final_calc_1mm(c: &mut Criterion) {
    final_calc!(c, MAX_DATA_SIZE);
}

criterion_group!(
    benches,
    update_10,
    update_1mm,
    final_calc_10,
    final_calc_1mm,
);
