use crate::batch;
use crate::chk;
use crate::error::StatsError::{InvalidData, NotEnoughData, Undefined};

// Test the batch functions. Calculate the descriptive stats on the whole array.
static ZEROS: [f64; 10] = [0.0; 10];
static ONES: [f64; 10] = [1.0; 10];
static ASCENDING: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

#[test]
fn test_batch_validate() {
    assert_eq!(batch::validate(&vec![]), Ok(()));
    assert_eq!(batch::validate(&vec![0.1]), Ok(()));
    assert_eq!(batch::validate(&vec![f64::NAN]), Err(InvalidData));
    assert_eq!(batch::validate(&vec![f64::INFINITY]), Err(InvalidData));
    assert_eq!(batch::validate(&vec![f64::NEG_INFINITY]), Err(InvalidData));
    assert_eq!(batch::validate(&vec![0.0, f64::NAN]), Err(InvalidData));
    assert_eq!(batch::validate(&vec![0.0, f64::INFINITY]), Err(InvalidData));
    assert_eq!(
        batch::validate(&vec![0.0, f64::NEG_INFINITY]),
        Err(InvalidData)
    );
}

#[test]
fn test_batch_stats_empty() {
    let a = vec![];
    chk!(batch::count(&a), 0u32);
    // Not enough data to define anything.
    chk!(batch::min(&a), Err(NotEnoughData));
    chk!(batch::max(&a), Err(NotEnoughData));
    chk!(batch::sum(&a), Err(NotEnoughData));
    chk!(batch::mean(&a), Err(NotEnoughData));
    chk!(batch::population_variance(&a), Err(NotEnoughData));
    chk!(batch::sample_variance(&a), Err(NotEnoughData));
    chk!(batch::population_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::sample_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::population_skew(&a), Err(NotEnoughData));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Err(NotEnoughData));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_1_zero() {
    let a = &ZEROS[..1];
    chk!(batch::count(&a), 1u32);
    // With one value, the first moment (mean) is available.
    chk!(batch::min(&a), Ok(0.0));
    chk!(batch::max(&a), Ok(0.0));
    chk!(batch::sum(&a), Ok(0.0));
    chk!(batch::mean(&a), Ok(0.0));
    chk!(batch::population_variance(&a), Err(NotEnoughData));
    chk!(batch::sample_variance(&a), Err(NotEnoughData));
    chk!(batch::population_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::sample_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::population_skew(&a), Err(NotEnoughData));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Err(NotEnoughData));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_2_zeros() {
    let a = &ZEROS[..2];
    chk!(batch::count(&a), 2u32);
    chk!(batch::min(&a), Ok(0.0));
    chk!(batch::max(&a), Ok(0.0));
    chk!(batch::sum(&a), Ok(0.0));
    chk!(batch::mean(&a), Ok(0.0));
    // With two values, the second moment (variance) is available.
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_zeros() {
    let a = &ZEROS[..3];
    chk!(batch::count(&a), 3u32);
    chk!(batch::min(&a), Ok(0.0));
    chk!(batch::max(&a), Ok(0.0));
    chk!(batch::sum(&a), Ok(0.0));
    chk!(batch::mean(&a), Ok(0.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    // With three values, the third moment (skew) is available, but because it's all zeros, they're
    // undefined.
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_4_zeros() {
    let a = &ZEROS[..4];
    chk!(batch::count(&a), 4u32);
    chk!(batch::min(&a), Ok(0.0));
    chk!(batch::max(&a), Ok(0.0));
    chk!(batch::sum(&a), Ok(0.0));
    chk!(batch::mean(&a), Ok(0.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    // With four values, the fourth moment (kurtosis) is available, but because it's all zeros,
    // they're undefined.
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(Undefined));
}
#[test]
fn test_batch_stats_5_zeros() {
    let a = &ZEROS[..5];
    chk!(batch::count(&a), 5u32);
    chk!(batch::min(&a), Ok(0.0));
    chk!(batch::max(&a), Ok(0.0));
    chk!(batch::sum(&a), Ok(0.0));
    chk!(batch::mean(&a), Ok(0.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(Undefined));
}

#[test]
fn test_batch_stats_1_one() {
    let a = &ONES[..1];
    chk!(batch::count(&a), 1u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(1.0));
    chk!(batch::sum(&a), Ok(1.0));
    chk!(batch::mean(&a), Ok(1.0));
    chk!(batch::population_variance(&a), Err(NotEnoughData));
    chk!(batch::sample_variance(&a), Err(NotEnoughData));
    chk!(batch::population_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::sample_standard_deviation(&a), Err(NotEnoughData));
    chk!(batch::population_skew(&a), Err(NotEnoughData));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Err(NotEnoughData));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_2_ones() {
    let a = &ONES[..2];
    chk!(batch::count(&a), 2u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(1.0));
    chk!(batch::sum(&a), Ok(2.0));
    chk!(batch::mean(&a), Ok(1.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_ones() {
    let a = &ONES[..3];
    chk!(batch::count(&a), 3u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(1.0));
    chk!(batch::sum(&a), Ok(3.0));
    chk!(batch::mean(&a), Ok(1.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    // With three values, the third moment (skew) is available, but because it's all ones, the
    // variance is 0.0, so they're undefined.
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_4_ones() {
    let a = &ONES[..4];
    chk!(batch::count(&a), 4u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(1.0));
    chk!(batch::sum(&a), Ok(4.0));
    chk!(batch::mean(&a), Ok(1.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(Undefined));
}
#[test]
fn test_batch_stats_5_ones() {
    let a = &ONES[..5];
    chk!(batch::count(&a), 5u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(1.0));
    chk!(batch::sum(&a), Ok(5.0));
    chk!(batch::mean(&a), Ok(1.0));
    chk!(batch::population_variance(&a), Ok(0.0));
    chk!(batch::sample_variance(&a), Ok(0.0));
    chk!(batch::population_standard_deviation(&a), Ok(0.0));
    chk!(batch::sample_standard_deviation(&a), Ok(0.0));
    chk!(batch::population_skew(&a), Err(Undefined));
    chk!(batch::sample_skew(&a), Err(Undefined));
    chk!(batch::population_kurtosis(&a), Err(Undefined));
    chk!(batch::sample_kurtosis(&a), Err(Undefined));
}
#[test]
fn test_batch_stats_2_ascending() {
    let a = &ASCENDING[..2];
    chk!(batch::count(&a), 2u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(2.0));
    chk!(batch::sum(&a), Ok(3.0));
    chk!(batch::mean(&a), Ok(1.5));
    chk!(batch::population_variance(&a), Ok(0.25));
    chk!(batch::sample_variance(&a), Ok(0.5));
    chk!(batch::population_standard_deviation(&a), Ok(0.5));
    chk!(batch::sample_standard_deviation(&a), Ok(0.7071067811865476));
    chk!(batch::population_skew(&a), Ok(0.0));
    chk!(batch::sample_skew(&a), Err(NotEnoughData));
    chk!(batch::population_kurtosis(&a), Ok(-2.0));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}

#[test]
fn test_batch_stats_3_ascending() {
    let a = &ASCENDING[..3];
    chk!(batch::count(&a), 3u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(3.0));
    chk!(batch::sum(&a), Ok(6.0));
    chk!(batch::mean(&a), Ok(2.0));
    chk!(batch::population_variance(&a), Ok(0.6666666666666666));
    chk!(batch::sample_variance(&a), Ok(1.0));
    chk!(
        batch::population_standard_deviation(&a),
        Ok(0.816496580927726)
    );
    chk!(batch::sample_standard_deviation(&a), Ok(1.0));
    // With three values, the third moment (skew) is available, but because the data is linear,
    // the skew is 0.0.
    chk!(batch::population_skew(&a), Ok(0.0));
    chk!(batch::sample_skew(&a), Ok(0.0));
    chk!(batch::population_kurtosis(&a), Ok(-1.5));
    chk!(batch::sample_kurtosis(&a), Err(NotEnoughData));
}
#[test]
fn test_batch_stats_4_ascending() {
    let a = &ASCENDING[..4];
    chk!(batch::count(&a), 4u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(4.0));
    chk!(batch::sum(&a), Ok(10.0));
    chk!(batch::mean(&a), Ok(2.5));
    chk!(batch::population_variance(&a), Ok(1.25));
    chk!(batch::sample_variance(&a), Ok(1.6666666666666667));
    chk!(
        batch::population_standard_deviation(&a),
        Ok(1.118033988749895)
    );
    chk!(batch::sample_standard_deviation(&a), Ok(1.2909944487358056));
    chk!(batch::population_skew(&a), Ok(0.0));
    chk!(batch::sample_skew(&a), Ok(0.0));
    chk!(batch::population_kurtosis(&a), Ok(-1.36));
    chk!(batch::sample_kurtosis(&a), Ok(-1.2));
}
#[test]
fn test_batch_stats_5_ascending() {
    let a = &ASCENDING[..5];
    chk!(batch::count(&a), 5u32);
    chk!(batch::min(&a), Ok(1.0));
    chk!(batch::max(&a), Ok(5.0));
    chk!(batch::sum(&a), Ok(15.0));
    chk!(batch::mean(&a), Ok(3.0));
    chk!(batch::population_variance(&a), Ok(2.0));
    chk!(batch::sample_variance(&a), Ok(2.5));
    chk!(
        batch::population_standard_deviation(&a),
        Ok(1.4142135623730951)
    );
    chk!(batch::sample_standard_deviation(&a), Ok(1.5811388300841898));
    chk!(batch::population_skew(&a), Ok(0.0));
    chk!(batch::sample_skew(&a), Ok(0.0));
    chk!(batch::population_kurtosis(&a), Ok(-1.3));
    chk!(batch::sample_kurtosis(&a), Ok(-1.2));
}

#[test]
fn test_batch_stats_ten_values() {
    // This array is in both the incremental and batch tests.
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    chk!(batch::count(&a), 10);
    chk!(batch::min(&a), Ok(-123.4));
    chk!(batch::max(&a), Ok(115.0));
    chk!(batch::sum(&a), Ok(62.83));
    chk!(batch::mean(&a), Ok(6.283));
    chk!(batch::population_variance(&a), Ok(3165.193161));
    chk!(batch::sample_variance(&a), Ok(3516.88129));
    chk!(
        batch::population_standard_deviation(&a),
        Ok(56.26004942230321)
    );
    chk!(batch::sample_standard_deviation(&a), Ok(59.3032991493728));
    chk!(batch::population_skew(&a), Ok(-0.4770396201629045));
    chk!(batch::sample_skew(&a), Ok(-0.565699400196136));
    chk!(batch::population_kurtosis(&a), Ok(1.253240236214162));
    chk!(batch::sample_kurtosis(&a), Ok(3.179835417592894));
}
