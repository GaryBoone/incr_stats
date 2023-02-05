use crate::chk;
use crate::tests::check::Checker;
use crate::Stats;

#[test]
fn test_update() {
    let mut d = Stats::new();
    d.update(2.3);
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
// Update() 2 values
fn test_update2() {
    let mut d = Stats::new();
    d.update(2.3);
    d.update(0.4);
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
// Update() 3 values.
fn test_update3() {
    let mut d = Stats::new();
    d.update(2.3);
    d.update(0.4);
    d.update(-3.4);
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
// Update() 4 values. Now all of the statistics are available.
fn test_update4() {
    let mut d = Stats::new();

    d.update(1.0);
    d.update(2.0);
    d.update(3.0);
    d.update(4.0);
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
// Update() 5 values.
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
// Update() 10 values.
fn test_update10() {
    let mut _d = Stats::new();

    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for _, v := range a {
    // 		d.Update(v)
    // 	}
    // chk!(d.count(), 10);
    // check_f64(d.min(), -123.4, "min");
    // check_f64(d.max(), 115.0, "max");
    // check_f64(d.sum(), 62.83, "sum");
    // check_f64(d.mean(), 6.283, "mean");
    // check_f64(
    //     d.population_variance().unwrap(),
    //     3165.19316100,
    //     "population_variance",
    // );
    // check_f64(d.sample_variance(), 3516.88129, "sample_variance");
    // check_f64(
    //     d.population_standard_deviation(),
    //     56.2600494223032,
    //     "population_standard_deviation",
    // );
    // check_f64(
    //     d.sample_standard_deviation(),
    //     59.3032991493728,
    //     "sample_standard_deviation",
    // );
    // check_f64(d.population_skew().unwrap(), -0.4770396201629045, "population_skew");
    // check_f64(d.sample_skew().unwrap(), -0.565699400196136, "sample_skew");
    // check_f64(
    //     d.population_kurtosis().unwrap(),
    //     1.253240236214162,
    //     "population_kurtosis",
    // );
    // check_f64(d.sample_kurtosis(), 3.179835417592894, "sample_kurtosis");
}

//
//
// Degenerate examples tests
//
//

#[test]
// Update() 1 0 value
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
// Update() 2 0 values
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
// Update() 3 0 values.
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
// Update() 4 0 values.
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
// Update() 5 0 values.
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
// Update() 10 0 values.
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
