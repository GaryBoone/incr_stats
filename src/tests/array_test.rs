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
    d.array_update(&a[..5]).unwrap();
    chk!(d.count(), 5);
    chk!(d.min(), Ok(-2.0));
    chk!(d.max(), Ok(115.0));
    chk!(d.sum(), Ok(174.0));
    chk!(d.mean(), Ok(34.8));
    chk!(d.population_variance(), Ok(1910.56));
    chk!(d.sample_variance(), Ok(2388.2));
    chk!(d.population_standard_deviation(), Ok(43.70995309995196));
    chk!(d.sample_standard_deviation(), Ok(48.869213212410116));
    chk!(d.population_skewness(), Ok(1.003118841855798));
    chk!(d.sample_skewness(), Ok(1.495361279933617));
    chk!(d.population_kurtosis(), Ok(-0.5476524250400354));
    chk!(d.sample_kurtosis(), Ok(1.809390299839858));

    // Load rest of array. The results will be cumulative.
    d.array_update(&a[5..]).unwrap();
    chk!(d.count(), 10);
    chk!(d.min(), Ok(-123.4));
    chk!(d.max(), Ok(115.0));
    chk!(d.sum(), Ok(62.83));
    chk!(d.mean(), Ok(6.283));
    chk!(d.population_variance(), Ok(3165.19316100));
    chk!(d.sample_variance(), Ok(3516.88129));
    chk!(d.population_standard_deviation(), Ok(56.26004942230321));
    chk!(d.sample_standard_deviation(), Ok(59.3032991493728));
    chk!(d.population_skewness(), Ok(-0.4770396201629045));
    chk!(d.sample_skewness(), Ok(-0.565699400196136));
    chk!(d.population_kurtosis(), Ok(1.253240236214162));
    chk!(d.sample_kurtosis(), Ok(3.179835417592894));
}
