use criterion::{black_box, criterion_group, Criterion};
use incr_stats::{batch, incr, vec};
use rand::Rng;

const MAX_DATA_SIZE: u32 = 1_000_000;

macro_rules! mean {
    ($c:expr, $count:expr) => {
        let mut rng = rand::thread_rng();
        let mut a = vec![];
        for _ in 0..$count {
            a.push(rng.gen())
        }
        $c.bench_function(&format!("mean_{}_incr", $count), |b| {
            b.iter(|| {
                let mut d = incr::Stats::new();
                for v in &a {
                    black_box(d.update(black_box(*v)).unwrap())
                }
                black_box(black_box(d).mean().unwrap());
            })
        });

        $c.bench_function(&format!("mean_{}_batch", $count), |b| {
            b.iter(|| black_box(batch::mean(black_box(&a)).unwrap()))
        });

        $c.bench_function(&format!("mean_{}_vec", $count), |b| {
            // Note: It would not be valid to do:
            //     let mut d = vec::Stats::new(&a).unwrap();
            //     b.iter(|| black_box(d.mean().unwrap()))
            // because that would be equivalent to
            //     let m = d.mean().unwrap();
            //     b.iter(|| black_box(m))
            // which means the compiler can optimize by calculating m once, outside the loop. Thus m
            // is effectively cached and the blackbox() measurement only times the cache lookup. Instead,
            // make sure to black_box() the data in the new() function and elsewhere.
            b.iter(|| black_box(vec::Stats::new(black_box(&a)).unwrap().mean()))
        });
    };
}

macro_rules! sample_kurtosis {
    ($c:expr, $count:expr) => {
        let mut rng = rand::thread_rng();
        let mut a = vec![];
        for _ in 0..$count {
            a.push(rng.gen())
        }
        $c.bench_function(&format!("kurtosis_{}_incr", $count), |b| {
            b.iter(|| {
                let mut d = incr::Stats::new();
                for v in &a {
                    black_box(d.update(black_box(*v)).unwrap())
                }
                black_box(d.sample_kurtosis().unwrap());
            })
        });

        $c.bench_function(&format!("kurtosis_{}_batch", $count), |b| {
            b.iter(|| black_box(batch::sample_kurtosis(black_box(&a)).unwrap()))
        });

        $c.bench_function(&format!("kurtosis_{}_vec", $count), |b| {
            b.iter(|| black_box(vec::Stats::new(black_box(&a)).unwrap().sample_kurtosis()))
        });
    };
}

pub fn mean_10(c: &mut Criterion) {
    mean!(c, 10);
}

pub fn mean_1mm(c: &mut Criterion) {
    mean!(c, MAX_DATA_SIZE);
}

pub fn kurtosis_10(c: &mut Criterion) {
    sample_kurtosis!(c, 10);
}

pub fn kurtosis_1mm(c: &mut Criterion) {
    sample_kurtosis!(c, MAX_DATA_SIZE);
}

criterion_group!(benches, mean_10, mean_1mm, kurtosis_10, kurtosis_1mm,);
