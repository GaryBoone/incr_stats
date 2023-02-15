use criterion::{black_box, criterion_group, Criterion};
use incr_stats::{batch, error::Result, incr, vec};
use rand::Rng;

const MAX_DATA_SIZE: u32 = 1_000_000;

// This benchmark compares incremental vs batch calculations for the situation in which the
// application calculates all of the descriptive statistics. Due to the redundancy in the batch
// calculations, such as the mean being calculated separately for each, the incremental stats are
// faster.

fn incr_all_stats(d: &incr::Stats) -> Result<()> {
    let _ = d.count();
    let _ = d.min()?;
    let _ = d.max()?;
    let _ = d.sum()?;
    let _ = d.mean()?;
    let _ = d.population_variance()?;
    let _ = d.sample_variance()?;
    let _ = d.population_standard_deviation()?;
    let _ = d.sample_standard_deviation()?;
    let _ = d.population_skewness()?;
    let _ = d.sample_skewness()?;
    let _ = d.population_kurtosis()?;
    let _ = d.sample_kurtosis()?;
    Ok(())
}

fn batch_all_stats(a: &[f64]) -> Result<()> {
    let _ = batch::count(&a);
    let _ = batch::min(&a)?;
    let _ = batch::max(&a)?;
    let _ = batch::sum(&a)?;
    let _ = batch::mean(&a)?;
    let _ = batch::population_variance(&a)?;
    let _ = batch::sample_variance(&a)?;
    let _ = batch::population_standard_deviation(&a)?;
    let _ = batch::sample_standard_deviation(&a)?;
    let _ = batch::population_skewness(&a)?;
    let _ = batch::sample_skewness(&a)?;
    let _ = batch::population_kurtosis(&a)?;
    let _ = batch::sample_kurtosis(&a)?;
    Ok(())
}

fn vec_all_stats(a: &[f64]) -> Result<()> {
    let mut d = vec::Stats::new(&a)?;
    let _ = d.count();
    let _ = d.min()?;
    let _ = d.max()?;
    let _ = d.sum()?;
    let _ = d.mean()?;
    let _ = d.population_variance()?;
    let _ = d.sample_variance()?;
    let _ = d.population_standard_deviation()?;
    let _ = d.sample_standard_deviation()?;
    let _ = d.population_skewness()?;
    let _ = d.sample_skewness()?;
    let _ = d.population_kurtosis()?;
    let _ = d.sample_kurtosis()?;
    Ok(())
}

#[macro_export]
macro_rules! all_stats {
    ($c:expr, $count:expr) => {
        let mut rng = rand::thread_rng();
        let mut a = vec![];
        for _ in 0..$count {
            a.push(rng.gen())
        }
        $c.bench_function(&format!("all_stats_{}_incr", $count), |b| {
            b.iter(|| {
                let mut d = incr::Stats::new();
                for v in &a {
                    black_box(d.update(black_box(*v)).unwrap())
                }
                black_box(incr_all_stats(black_box(&d)).unwrap());
            })
        });

        $c.bench_function(&format!("all_stats_{}_batch", $count), |b| {
            b.iter(|| black_box(batch_all_stats(black_box(&a)).unwrap()))
        });

        $c.bench_function(&format!("all_stats_{}_vec", $count), |b| {
            b.iter(|| black_box(vec_all_stats(black_box(&a)).unwrap()))
        });
    };
}

pub fn all_stats_10(c: &mut Criterion) {
    all_stats!(c, 10);
}

pub fn all_stats_1mm(c: &mut Criterion) {
    all_stats!(c, MAX_DATA_SIZE);
}

criterion_group!(benches, all_stats_10, all_stats_1mm);
