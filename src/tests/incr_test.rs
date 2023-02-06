use crate::chk;
use crate::incr::Stats;

#[test]
fn test_update0() {
    let d = Stats::new();
    // With no values added, the first moment, the mean, is zero and none of the other moments are
    // defined.
    chk!(d.count(), 0);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance(), None);
    chk!(d.sample_variance(), None);
    chk!(d.population_standard_deviation(), None);
    chk!(d.sample_standard_deviation(), None);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
fn test_update1() {
    let mut d = Stats::new();
    d.update(2.3);
    // With one value added, the first moment, the mean, exists but none of the other moments are
    // defined.
    chk!(d.count(), 1u32);
    chk!(d.min(), 2.3);
    chk!(d.max(), 2.3);
    chk!(d.sum(), 2.3);
    chk!(d.mean(), 2.3);
    chk!(d.population_variance(), None);
    chk!(d.sample_variance(), None);
    chk!(d.population_standard_deviation(), None);
    chk!(d.sample_standard_deviation(), None);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 2 values
fn test_update2() {
    let mut d = Stats::new();
    d.update(2.3);
    d.update(0.4);
    // With two values added, the first two moments exist: the mean and the variance. The next two,
    // the skew and kurtosis, are defined for the population, but not for samples.
    chk!(d.count(), 2u32);
    chk!(d.min(), 0.4);
    chk!(d.max(), 2.3);
    chk!(d.sum(), 2.7);
    chk!(d.mean(), 1.35);
    chk!(d.population_variance().unwrap(), 0.9025);
    chk!(d.sample_variance().unwrap(), 1.805);
    chk!(d.population_standard_deviation().unwrap(), 0.95);
    chk!(d.sample_standard_deviation().unwrap(), 1.34350288425444);
    chk!(d.population_skew().unwrap(), 0.0);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis().unwrap(), -2.0);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 3 values.
fn test_update3() {
    let mut d = Stats::new();
    d.update(2.3);
    d.update(0.4);
    d.update(-3.4);
    // With three values added, the first three moments exist: the mean, the variance, and the skew
    // are defined. The population kurtosis exists, but not the sample kurtosis.
    chk!(d.count(), 3u32);
    chk!(d.min(), -3.4);
    chk!(d.max(), 2.3);
    chk!(d.sum(), -0.7);
    chk!(d.mean(), -0.2333333333333334);
    chk!(d.population_variance().unwrap(), 5.615555555555554);
    chk!(d.sample_variance().unwrap(), 8.42333333333333);
    chk!(d.population_standard_deviation().unwrap(), 2.36971634495683);
    chk!(d.sample_standard_deviation().unwrap(), 2.90229794013870);
    chk!(d.population_skew().unwrap(), -0.3818017741606063);
    chk!(d.sample_skew(), Some(-0.9352195295828242));
    chk!(d.population_kurtosis().unwrap(), -1.5);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 4 values.
fn test_update4() {
    let mut d = Stats::new();
    d.update(1.0);
    d.update(2.0);
    d.update(3.0);
    d.update(4.0);
    // With four values added, all of the first four moments of the statistics are available: the
    // mean, the variance, the skew, and the kurtosis, both populations and samples. Note that this
    // is not always the case, as the all-zeros case below shows.
    chk!(d.count(), 4u32);
    chk!(d.min(), 1.0);
    chk!(d.max(), 4.0);
    chk!(d.sum(), 10.0);
    chk!(d.mean(), 2.5);
    chk!(d.population_variance().unwrap(), 1.25);
    chk!(d.sample_variance().unwrap(), 1.666666666666667);
    chk!(
        d.population_standard_deviation().unwrap(),
        1.118033988749895
    );
    chk!(d.sample_standard_deviation().unwrap(), 1.290994448735806);
    chk!(d.population_skew().unwrap(), 0.0);
    chk!(d.sample_skew().unwrap(), 0.0);
    chk!(d.population_kurtosis().unwrap(), -1.36);
    chk!(d.sample_kurtosis().unwrap(), -1.2);
}

#[test]
// Call update() with 5 values.
fn test_update5() {
    let mut d = Stats::new();

    d.update(1.0);
    d.update(2.0);
    d.update(3.0);
    d.update(4.0);
    d.update(5.0);
    chk!(d.count(), 5u32);
    chk!(d.min(), 1.0);
    chk!(d.max(), 5.0);
    chk!(d.sum(), 15.0);
    chk!(d.mean(), 3.0);
    chk!(d.population_variance().unwrap(), 2.0);
    chk!(d.sample_variance().unwrap(), 2.5);
    chk!(
        d.population_standard_deviation().unwrap(),
        1.414213562373095
    );
    chk!(d.sample_standard_deviation().unwrap(), 1.5811388300841898);
    chk!(d.population_skew().unwrap(), 0.0);
    chk!(d.sample_skew().unwrap(), 0.0);
    chk!(d.population_kurtosis().unwrap(), -1.3);
    chk!(d.sample_kurtosis().unwrap(), -1.2);
}

#[test]
// Call update() with 10 values.
fn test_update10() {
    let mut d = Stats::new();
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    for v in a {
        d.update(v);
    }
    chk!(d.count(), 10);
    chk!(d.min(), -123.4);
    chk!(d.max(), 115.0);
    chk!(d.sum(), 62.83);
    chk!(d.mean(), 6.283);
    chk!(d.population_variance().unwrap(), 3165.19316100);
    chk!(d.sample_variance().unwrap(), 3516.88129);
    chk!(
        d.population_standard_deviation().unwrap(),
        56.26004942230321
    );
    chk!(d.sample_standard_deviation().unwrap(), 59.3032991493728);
    chk!(d.population_skew().unwrap(), -0.4770396201629045);
    chk!(d.sample_skew().unwrap(), -0.565699400196136);
    chk!(d.population_kurtosis().unwrap(), 1.253240236214162);
    chk!(d.sample_kurtosis().unwrap(), 3.179835417592894);
}

//
//
// Degenerate examples tests
//
//
#[test]
// Call update() with 1 zero value.
fn test_update01() {
    let mut d = Stats::new();

    d.update(0.0);
    chk!(d.count(), 1u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance(), None);
    chk!(d.sample_variance(), None);
    chk!(d.population_standard_deviation(), None);
    chk!(d.sample_standard_deviation(), None);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 2 zero values.
fn test_update02() {
    let mut d = Stats::new();

    d.update(0.0);
    d.update(0.0);
    chk!(d.count(), 2u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance().unwrap(), 0.0);
    chk!(d.sample_variance().unwrap(), 0.0);
    chk!(d.population_standard_deviation().unwrap(), 0.0);
    chk!(d.sample_standard_deviation().unwrap(), 0.0);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 3 zero values.
fn test_update03() {
    let mut d = Stats::new();

    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    chk!(d.count(), 3u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance().unwrap(), 0.0);
    chk!(d.sample_variance().unwrap(), 0.0);
    chk!(d.population_standard_deviation().unwrap(), 0.0);
    chk!(d.sample_standard_deviation().unwrap(), 0.0);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 4 zero values.
fn test_update04() {
    let mut d = Stats::new();

    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    chk!(d.count(), 4u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance().unwrap(), 0.0);
    chk!(d.sample_variance().unwrap(), 0.0);
    chk!(d.population_standard_deviation().unwrap(), 0.0);
    chk!(d.sample_standard_deviation().unwrap(), 0.0);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 5 zero values.
fn test_update05() {
    let mut d = Stats::new();

    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    d.update(0.0);
    chk!(d.count(), 5u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance().unwrap(), 0.0);
    chk!(d.sample_variance().unwrap(), 0.0);
    chk!(d.population_standard_deviation().unwrap(), 0.0);
    chk!(d.sample_standard_deviation().unwrap(), 0.0);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}

#[test]
// Call update() with 10 zero values.
fn test_update010() {
    let mut d = Stats::new();

    let a = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    for v in a {
        d.update(v)
    }
    chk!(d.count(), 10u32);
    chk!(d.min(), 0.0);
    chk!(d.max(), 0.0);
    chk!(d.sum(), 0.0);
    chk!(d.mean(), 0.0);
    chk!(d.population_variance().unwrap(), 0.0);
    chk!(d.sample_variance().unwrap(), 0.0);
    chk!(d.population_standard_deviation().unwrap(), 0.0);
    chk!(d.sample_standard_deviation().unwrap(), 0.0);
    chk!(d.population_skew(), None);
    chk!(d.sample_skew(), None);
    chk!(d.population_kurtosis(), None);
    chk!(d.sample_kurtosis(), None);
}
