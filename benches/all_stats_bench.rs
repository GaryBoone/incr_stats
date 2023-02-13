use criterion::{black_box, criterion_group, Criterion};
use incr_stats::{batch, desc::Desc, error::Result, incr::Stats};
use rand::Rng;

// This benchmark compares incremental vs batch calculations for the situation in which the
// application calculates all of the descriptive statistics. Due to the redundancy in the batch
// calculations, such as the mean being calculated separately for each, the incremental stats are
// faster.

fn incr_all_stats(d: &Stats) -> Result<()> {
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

fn desc_all_stats(d: &mut Desc) -> Result<()> {
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

pub fn all_stats_1000(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut a = vec![];
    for _ in 0..1000 {
        a.push(rng.gen())
    }
    // Incremental
    c.bench_function("all_stats_1000_incr", |b| {
        b.iter(|| {
            let mut d = Stats::new();
            for v in &a {
                d.update(black_box(*v)).unwrap()
            }
            black_box(incr_all_stats(&d)).unwrap();
        })
    });

    // Batch
    c.bench_function("all_stats_1000_batch", |b| {
        b.iter(|| batch_all_stats(black_box(&a)).unwrap())
    });

    // Desc
    c.bench_function("all_stats_1000_desc", |b| {
        let mut d = Desc::new(&a).unwrap();
        b.iter(|| desc_all_stats(black_box(&mut d)).unwrap())
    });
}

criterion_group!(benches, all_stats_1000,);
