use crate::chk;
use crate::error::StatsError::{InvalidData, NotEnoughData, Undefined};
use crate::vec::Stats;

// Test the incremtal functions. Update the descriptive stats one point at a time.
static ZEROS: [f64; 10] = [0.0; 10];
static ONES: [f64; 10] = [1.0; 10];
static ASCENDING: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
static VALUES: [f64; 10] = [
    1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
];

#[test]
fn test_update_with_bad_data() {
    assert_eq!(Stats::new(&vec![f64::NAN]), Err(InvalidData));
    assert_eq!(Stats::new(&vec![f64::INFINITY]), Err(InvalidData));
    assert_eq!(Stats::new(&vec![f64::NEG_INFINITY]), Err(InvalidData));
    assert_eq!(Stats::new(&vec![0.0, f64::NAN]), Err(InvalidData));
    assert_eq!(Stats::new(&vec![0.0, f64::INFINITY]), Err(InvalidData));
    assert_eq!(Stats::new(&vec![0.0, f64::NEG_INFINITY]), Err(InvalidData));
}

#[test]
fn test_update_empty() {
    let empty = vec![];
    let mut d = Stats::new(&empty).unwrap();
    // With no values added, the first moment, the mean, is zero and none of the other moments are
    // defined.
    chk!(d.count(), 0);
    chk!(d.min(), Err(NotEnoughData));
    chk!(d.max(), Err(NotEnoughData));
    chk!(d.sum(), Err(NotEnoughData));
    chk!(d.mean(), Err(NotEnoughData));
    chk!(d.population_variance(), Err(NotEnoughData));
    chk!(d.sample_variance(), Err(NotEnoughData));
    chk!(d.population_standard_deviation(), Err(NotEnoughData));
    chk!(d.sample_standard_deviation(), Err(NotEnoughData));
    chk!(d.population_skewness(), Err(NotEnoughData));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Err(NotEnoughData));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_1_zero() {
    let mut d = Stats::new(&ZEROS[..1]).unwrap();
    chk!(d.count(), 1u32);
    // With one value, the first moment (mean) is available.
    chk!(d.min(), Ok(0.0));
    chk!(d.max(), Ok(0.0));
    chk!(d.sum(), Ok(0.0));
    chk!(d.mean(), Ok(0.0));
    chk!(d.population_variance(), Err(NotEnoughData));
    chk!(d.sample_variance(), Err(NotEnoughData));
    chk!(d.population_standard_deviation(), Err(NotEnoughData));
    chk!(d.sample_standard_deviation(), Err(NotEnoughData));
    chk!(d.population_skewness(), Err(NotEnoughData));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Err(NotEnoughData));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_2_zeros() {
    let mut d = Stats::new(&ZEROS[..2]).unwrap();
    chk!(d.count(), 2u32);
    chk!(d.min(), Ok(0.0));
    chk!(d.max(), Ok(0.0));
    chk!(d.sum(), Ok(0.0));
    chk!(d.mean(), Ok(0.0));
    // With two values, the second moment (variance) is available.
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_zeros() {
    let mut d = Stats::new(&ZEROS[..3]).unwrap();
    chk!(d.count(), 3u32);
    chk!(d.min(), Ok(0.0));
    chk!(d.max(), Ok(0.0));
    chk!(d.sum(), Ok(0.0));
    chk!(d.mean(), Ok(0.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    // With three values, the third moment (skew) is available, but because it's all zeros, they're
    // undefined.
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_4_zeros() {
    let mut d = Stats::new(&ZEROS[..4]).unwrap();
    chk!(d.count(), 4u32);
    chk!(d.min(), Ok(0.0));
    chk!(d.max(), Ok(0.0));
    chk!(d.sum(), Ok(0.0));
    chk!(d.mean(), Ok(0.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    // With four values, the fourth moment (kurtosis) is available, but because it's all zeros,
    // they're undefined.
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(Undefined));
}
#[test]
fn test_batch_stats_5_zeros() {
    let mut d = Stats::new(&ZEROS[..5]).unwrap();
    chk!(d.count(), 5u32);
    chk!(d.min(), Ok(0.0));
    chk!(d.max(), Ok(0.0));
    chk!(d.sum(), Ok(0.0));
    chk!(d.mean(), Ok(0.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(Undefined));
}

#[test]
fn test_batch_stats_1_one() {
    let mut d = Stats::new(&ONES[..1]).unwrap();
    chk!(d.count(), 1u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(1.0));
    chk!(d.sum(), Ok(1.0));
    chk!(d.mean(), Ok(1.0));
    chk!(d.population_variance(), Err(NotEnoughData));
    chk!(d.sample_variance(), Err(NotEnoughData));
    chk!(d.population_standard_deviation(), Err(NotEnoughData));
    chk!(d.sample_standard_deviation(), Err(NotEnoughData));
    chk!(d.population_skewness(), Err(NotEnoughData));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Err(NotEnoughData));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_2_ones() {
    let mut d = Stats::new(&ONES[..2]).unwrap();
    chk!(d.count(), 2u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(1.0));
    chk!(d.sum(), Ok(2.0));
    chk!(d.mean(), Ok(1.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_ones() {
    let mut d = Stats::new(&ONES[..3]).unwrap();
    chk!(d.count(), 3u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(1.0));
    chk!(d.sum(), Ok(3.0));
    chk!(d.mean(), Ok(1.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    // With three values, the third moment (skew) is available, but because it's all ones, the
    // variance is 0.0, so they're undefined.
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_4_ones() {
    let mut d = Stats::new(&ONES[..4]).unwrap();
    chk!(d.count(), 4u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(1.0));
    chk!(d.sum(), Ok(4.0));
    chk!(d.mean(), Ok(1.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(Undefined));
}
#[test]
fn test_batch_stats_5_ones() {
    let mut d = Stats::new(&ONES[..5]).unwrap();
    chk!(d.count(), 5u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(1.0));
    chk!(d.sum(), Ok(5.0));
    chk!(d.mean(), Ok(1.0));
    chk!(d.population_variance(), Ok(0.0));
    chk!(d.sample_variance(), Ok(0.0));
    chk!(d.population_standard_deviation(), Ok(0.0));
    chk!(d.sample_standard_deviation(), Ok(0.0));
    chk!(d.population_skewness(), Err(Undefined));
    chk!(d.sample_skewness(), Err(Undefined));
    chk!(d.population_kurtosis(), Err(Undefined));
    chk!(d.sample_kurtosis(), Err(Undefined));
}

#[test]
fn test_batch_stats_2_ascending() {
    let mut d = Stats::new(&ASCENDING[..2]).unwrap();
    chk!(d.count(), 2u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(2.0));
    chk!(d.sum(), Ok(3.0));
    chk!(d.mean(), Ok(1.5));
    chk!(d.population_variance(), Ok(0.25));
    chk!(d.sample_variance(), Ok(0.5));
    chk!(d.population_standard_deviation(), Ok(0.5));
    chk!(d.sample_standard_deviation(), Ok(0.7071067811865476));
    chk!(d.population_skewness(), Ok(0.0));
    chk!(d.sample_skewness(), Err(NotEnoughData));
    chk!(d.population_kurtosis(), Ok(-2.0));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_ascending() {
    let mut d = Stats::new(&ASCENDING[..3]).unwrap();
    chk!(d.count(), 3u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(3.0));
    chk!(d.sum(), Ok(6.0));
    chk!(d.mean(), Ok(2.0));
    chk!(d.population_variance(), Ok(0.6666666666666666));
    chk!(d.sample_variance(), Ok(1.0));
    chk!(d.population_standard_deviation(), Ok(0.816496580927726));
    chk!(d.sample_standard_deviation(), Ok(1.0));
    // With three values, the third moment (skew) is available, but because the data is linear,
    // the skew is 0.0.
    chk!(d.population_skewness(), Ok(0.0));
    chk!(d.sample_skewness(), Ok(0.0));
    chk!(d.population_kurtosis(), Ok(-1.5));
    chk!(d.sample_kurtosis(), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_4_ascending() {
    let mut d = Stats::new(&ASCENDING[..4]).unwrap();
    chk!(d.count(), 4u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(4.0));
    chk!(d.sum(), Ok(10.0));
    chk!(d.mean(), Ok(2.5));
    chk!(d.population_variance(), Ok(1.25));
    chk!(d.sample_variance(), Ok(1.6666666666666667));
    chk!(d.population_standard_deviation(), Ok(1.118033988749895));
    chk!(d.sample_standard_deviation(), Ok(1.2909944487358056));
    chk!(d.population_skewness(), Ok(0.0));
    chk!(d.sample_skewness(), Ok(0.0));
    chk!(d.population_kurtosis(), Ok(-1.36));
    chk!(d.sample_kurtosis(), Ok(-1.2));
}
#[test]
fn test_batch_stats_5_ascending() {
    let mut d = Stats::new(&ASCENDING[..5]).unwrap();
    chk!(d.count(), 5u32);
    chk!(d.min(), Ok(1.0));
    chk!(d.max(), Ok(5.0));
    chk!(d.sum(), Ok(15.0));
    chk!(d.mean(), Ok(3.0));
    chk!(d.population_variance(), Ok(2.0));
    chk!(d.sample_variance(), Ok(2.5));
    chk!(d.population_standard_deviation(), Ok(1.4142135623730951));
    chk!(d.sample_standard_deviation(), Ok(1.5811388300841898));
    chk!(d.population_skewness(), Ok(0.0));
    chk!(d.sample_skewness(), Ok(0.0));
    chk!(d.population_kurtosis(), Ok(-1.3));
    chk!(d.sample_kurtosis(), Ok(-1.2));
}

#[test]
// Call update() with 10 values that are also used in the batch tests.
fn test_update10() {
    let mut d = Stats::new(&VALUES).unwrap();
    chk!(d.count(), 10);
    chk!(d.min(), Ok(-123.4));
    chk!(d.max(), Ok(115.0));
    chk!(d.sum(), Ok(62.83));
    chk!(d.mean(), Ok(6.283));
    chk!(d.population_variance(), Ok(3165.19316100));
    chk!(d.sample_variance(), Ok(3516.8812900000003));
    chk!(d.population_standard_deviation(), Ok(56.26004942230321));
    chk!(d.sample_standard_deviation(), Ok(59.3032991493728));
    chk!(d.population_skewness(), Ok(-0.4770396201629045));
    chk!(d.sample_skewness(), Ok(-0.565699400196136));
    chk!(d.population_kurtosis(), Ok(1.253240236214162));
    chk!(d.sample_kurtosis(), Ok(3.179835417592894));
}
