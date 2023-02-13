use crate::batch;
use crate::chk;
use crate::desc::Desc;
use crate::incr::Stats;

// Check that the incremental and batch functions return identical results.
fn check_batch_v_incr(d: &Stats, a: &[f64]) {
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

// Check that the incremental and desc functions return identical results.
fn check_incr_v_desc(s: &Stats, a: &[f64]) {
    let mut d = Desc::new(a).unwrap();
    chk!(s.count(), d.count());
    chk!(s.min(), d.min());
    chk!(s.max(), d.max());
    chk!(s.sum(), d.sum());
    chk!(s.mean(), d.mean());
    chk!(s.population_variance(), d.population_variance());
    chk!(s.sample_variance(), d.sample_variance());
    chk!(
        s.population_standard_deviation(),
        d.population_standard_deviation()
    );
    chk!(s.sample_standard_deviation(), d.sample_standard_deviation());
    chk!(s.population_skewness(), d.population_skewness());
    chk!(s.sample_skewness(), d.sample_skewness());
    chk!(s.population_kurtosis(), d.population_kurtosis());
    chk!(s.sample_kurtosis(), d.sample_kurtosis());
}

// Calculate statistics with empty data.
#[test]
fn test_incr_vs_batch_update_empty() {
    let d = Stats::new();
    let a = vec![];
    check_batch_v_incr(&d, &a);
    check_incr_v_desc(&d, &a);
}

#[test]
// Calculate statistics with 0 to 10 values.
fn test_incr_vs_batch_update_10_slices() {
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_batch_v_incr(&d, &a[..i]);
        check_incr_v_desc(&d, &a[..i]);
    }
}

// Calculate statistics with 0 to 10 zeros.
#[test]
fn test_incr_vs_batch_update_for_zeros() {
    let a = vec![0.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_batch_v_incr(&d, &a[..i]);
        check_incr_v_desc(&d, &a[..i]);
    }
}

// Calculate statistics with 0 to 10 ones.
#[test]
fn test_incr_vs_batch_update_for_ones() {
    let a = vec![1.0; 10];
    // Confirm the incremental and batch versions match for all slices of a.
    for i in 0..a.len() {
        let mut d = Stats::new();
        a[..i].iter().for_each(|v| d.update(*v).unwrap());
        check_batch_v_incr(&d, &a[..i]);
        check_incr_v_desc(&d, &a[..i]);
    }
}
