use crate::batch::*;
use crate::chk;
use crate::incr::Stats;

// Check that the incremental and batch functions return identical results.
fn check_all(d: &Stats, a: &[f64]) {
    chk!(stats_count(&a), d.count());
    chk!(stats_min(&a), d.min());
    chk!(stats_max(&a), d.max());
    chk!(stats_sum(&a), d.sum());
    chk!(stats_mean(&a), d.mean());
    chk!(stats_population_variance(&a), d.population_variance());
    chk!(stats_sample_variance(&a), d.sample_variance());
    chk!(
        stats_population_standard_deviation(&a),
        d.population_standard_deviation()
    );
    chk!(
        stats_sample_standard_deviation(&a),
        d.sample_standard_deviation()
    );
    chk!(stats_population_skew(&a), d.population_skew());
    chk!(stats_sample_skew(&a), d.sample_skew());
    chk!(stats_population_kurtosis(&a), d.population_kurtosis());
    chk!(stats_sample_kurtosis(&a), d.sample_kurtosis());
}

#[test]
fn test_incr_vs_batch_update_empty() {
    let d = Stats::new();
    let a = vec![];
    check_all(&d, &a);
}

#[test]
// Call update() with 10 values.
fn test_incr_vs_batch_update_10_slices() {
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v));
        check_all(&d, &a[..i]);
    }
}

#[test]
fn test_incr_vs_batch_update_for_zeros() {
    let a = vec![0.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v));
        check_all(&d, &a[..i]);
    }
}

#[test]
fn test_incr_vs_batch_update_for_ones() {
    let a = vec![1.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v));
        check_all(&d, &a[..i]);
    }
}
