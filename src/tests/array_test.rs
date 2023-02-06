use crate::chk;
use crate::incr::Stats;

#[test]
// Update by array. In this case, we use slices to update via half of the array at a time.
fn test_update_array10() {
    let mut d = Stats::new();

    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    // Load the first half of the array
    d.update_array(&a[..5]);
    chk!(d.count(), 5);
    chk!(d.min(), -2.0);
    chk!(d.max(), 115.0);
    chk!(d.sum(), 174.0);
    chk!(d.mean(), 34.8);
    chk!(d.population_variance().unwrap(), 1910.56);
    chk!(d.sample_variance().unwrap(), 2388.2);
    chk!(
        d.population_standard_deviation().unwrap(),
        43.70995309995196
    );
    chk!(d.sample_standard_deviation().unwrap(), 48.869213212410116);
    chk!(d.population_skew().unwrap(), 1.003118841855798);
    chk!(d.sample_skew().unwrap(), 1.495361279933617);
    chk!(d.population_kurtosis().unwrap(), -0.5476524250400354);
    chk!(d.sample_kurtosis().unwrap(), 1.809390299839858);

    // Load rest of array. The results will be cumulative.
    d.update_array(&a[5..]);
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
