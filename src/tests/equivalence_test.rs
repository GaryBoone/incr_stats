use crate::batch;
use crate::chk;
use crate::incr::Stats;

// Check that the incremental and batch functions return identical results.
fn check_all(d: &Stats, a: &[f64]) {
    chk!(batch::count(&a), d.count());
    chk!(batch::min(&a), d.min());
    chk!(batch::max(&a), d.max());
    chk!(batch::sum(&a), d.sum());
    chk!(batch::mean(&a), d.mean());
    chk!(batch::population_variance(&a), d.population_variance());
    chk!(batch::sample_variance(&a), d.sample_variance());
    chk!(
        batch::population_standard_deviation(&a),
        d.population_standard_deviation()
    );
    chk!(
        batch::sample_standard_deviation(&a),
        d.sample_standard_deviation()
    );
    chk!(batch::population_skewness(&a), d.population_skewness());
    chk!(batch::sample_skewness(&a), d.sample_skewness());
    chk!(batch::population_kurtosis(&a), d.population_kurtosis());
    chk!(batch::sample_kurtosis(&a), d.sample_kurtosis());
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
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_all(&d, &a[..i]);
    }
}

#[test]
fn test_incr_vs_batch_update_for_zeros() {
    let a = vec![0.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_all(&d, &a[..i]);
    }
}

#[test]
fn test_incr_vs_batch_update_for_ones() {
    let a = vec![1.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_all(&d, &a[..i]);
    }
}
