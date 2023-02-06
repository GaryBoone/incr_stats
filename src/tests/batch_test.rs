use crate::batch::*;
use crate::chk;

#[test]
// Test the batch functions. Calculate the descriptive stats on the whole array.
fn test_batch_stats0() {
    chk!(stats_min(&vec![]), None);
}

#[test]
// Test the batch functions. Calculate the descriptive stats on the whole array.
fn test_batch_stats() {
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    chk!(stats_count(&a), 5);
    chk!(stats_min(&a).unwrap(), 1.0);
    chk!(stats_max(&a).unwrap(), 5.0);
    // chk!(stats_sum(a), 15.0);
    // chk!(stats_mean(a), 3.0);
    // chk!(stats_population_variance(a).unwrap(), 2.0);
    // chk!(stats_sample_variance(a).unwrap(), 2.5);
    // chk!(
    //     stats_population_standard_deviation(a).unwrap(),
    //     1.414213562373095
    // );
    // chk!(
    //     stats_sample_standard_deviation(a).unwrap(),
    //     1.5811388300841898
    // );
    // chk!(stats_population_skew(a).unwrap(), 0.0);
    // chk!(stats_sample_skew(a).unwrap(), 0.0);
    // chk!(stats_population_kurtosis(a).unwrap(), -1.3);
    // chk!(stats_sample_kurtosis(a).unwrap(), -1.2);
}

#[test]
fn test_batch_stats2() {
    let a = vec![
        1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3,
    ];
    chk!(stats_count(&a), 10);
    chk!(stats_min(&a).unwrap(), -123.4);
    //chk!(stats_max(a), 115.0);
    //chk!(stats_sum(a), 62.83);
    //chk!(stats_mean(a), 6.283);
    //chk!(stats_population_variance(a).unwrap(), 3165.19316100);
    //chk!(stats_sample_variance(a).unwrap(), 3516.88129);
    //chk!(
    //    stats_population_standard_deviation(a).unwrap(),
    //    56.2600494223032
    //);
    //chk!(
    //    stats_sample_standard_deviation(a).unwrap(),
    //    59.3032991493728
    //);
    //chk!(stats_population_skew(a).unwrap(), -0.4770396201629045);
    //chk!(stats_sample_skew(a).unwrap(), -0.565699400196136);
    //chk!(stats_population_kurtosis(a).unwrap(), 1.253240236214162);
    //chk!(stats_sample_kurtosis(a).unwrap(), 3.179835417592894);
}
