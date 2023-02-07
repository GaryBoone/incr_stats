use crate::batch::*;
use crate::chk;
use crate::error::StatsError;

// Test the batch functions. Calculate the descriptive stats on the whole array.

#[test]
fn test_batch_stats_empty() {
    let a = vec![];
    chk!(stats_count(&a), 0u32);
    chk!(stats_min(&a), Err(StatsError::NoData));
    chk!(stats_max(&a), Err(StatsError::NoData));
    chk!(stats_sum(&a), Err(StatsError::NoData));
    chk!(stats_mean(&a), Err(StatsError::NoData));
    chk!(stats_population_variance(&a), None);
    chk!(stats_sample_variance(&a), None);
    chk!(stats_population_standard_deviation(&a), None);
    chk!(stats_sample_standard_deviation(&a), None);
    chk!(stats_population_skew(&a), None);
    chk!(stats_sample_skew(&a), None);
    chk!(stats_population_kurtosis(&a), None);
    chk!(
        stats_sample_kurtosis(&a),
        Err(StatsError::FourthMomentUndefined)
    );
}

#[test]
fn test_batch_stats0() {
    let a = vec![0.0];
    chk!(stats_count(&a), 1u32);
    chk!(stats_min(&a), Ok(0.0));
    chk!(stats_max(&a), Ok(0.0));
    chk!(stats_sum(&a), Ok(0.0));
    chk!(stats_mean(&a), Ok(0.0));
    chk!(stats_population_variance(&a), Some(0.0));
    chk!(stats_sample_variance(&a), None);
    // chk!(stats_population_standard_deviation(&a), None);
    chk!(stats_sample_standard_deviation(&a), None);
    // chk!(stats_population_skew(&a), None);
    // chk!(stats_sample_skew(&a), None);
    // chk!(stats_population_kurtosis(&a), None);
    // chk!(stats_sample_kurtosis(&a), None);
}

fn test_batch_stats1() {
    chk!(stats_min(&vec![2.0]), Ok(2.0));
    // TODO: Add rest.
}

fn test_batch_stats2() {
    chk!(stats_min(&vec![-1.2, 2.0]), Ok(-1.2));
    // TODO: Add rest.
}

#[test]
fn test_batch_stats5() {
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    chk!(stats_count(&a), 5);
    chk!(stats_min(&a).unwrap(), 1.0);
    chk!(stats_max(&a).unwrap(), 5.0);
    chk!(stats_sum(&a).unwrap(), 15.0);
    chk!(stats_mean(&a).unwrap(), 3.0);
    chk!(stats_population_variance(&a).unwrap(), 2.0);
    chk!(stats_sample_variance(&a).unwrap(), 2.5);
    chk!(
        stats_population_standard_deviation(&a).unwrap(),
        1.414213562373095
    );
    chk!(
        stats_sample_standard_deviation(&a).unwrap(),
        1.5811388300841898
    );
    chk!(stats_population_skew(&a).unwrap(), 0.0);
    chk!(stats_sample_skew(&a).unwrap(), 0.0);
    chk!(stats_population_kurtosis(&a).unwrap(), -1.3);
    chk!(stats_sample_kurtosis(&a).unwrap(), -1.2);
}

#[test]
fn test_batch_stats10() {
    // This array is in both the incremental and batch tests.
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    chk!(stats_count(&a), 10);
    chk!(stats_min(&a).unwrap(), -123.4);
    chk!(stats_max(&a).unwrap(), 115.0);
    chk!(stats_sum(&a).unwrap(), 62.83);
    chk!(stats_mean(&a).unwrap(), 6.283);
    chk!(stats_population_variance(&a).unwrap(), 3165.19316100);
    chk!(stats_sample_variance(&a).unwrap(), 3516.88129);
    chk!(
        stats_population_standard_deviation(&a).unwrap(),
        56.26004942230321
    );
    chk!(
        stats_sample_standard_deviation(&a).unwrap(),
        59.3032991493728
    );
    chk!(stats_population_skew(&a).unwrap(), -0.4770396201629045);
    chk!(stats_sample_skew(&a).unwrap(), -0.565699400196136);
    chk!(stats_population_kurtosis(&a).unwrap(), 1.253240236214162);
    chk!(stats_sample_kurtosis(&a).unwrap(), 3.179835417592894);
}
