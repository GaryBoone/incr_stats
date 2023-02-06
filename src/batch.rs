//
//
// Batch functions
//
// These are non-incremental functions that operate only on the data given them.
// They're prefixed with 'Stats'.
//
pub fn stats_count(data: &[f64]) -> u32 {
    data.len() as u32
}

pub fn stats_min(data: &[f64]) -> Option<f64> {
    if data.len() == 0 {
        return None;
    }
    let mut min = data[0];
    for v in data {
        if v < &min {
            min = *v;
        }
    }
    Some(min)
}

pub fn stats_max(data: &[f64]) -> Option<f64> {
    if data.len() == 0 {
        return None;
    }
    let mut max = data[0];
    for v in data {
        if v > &max {
            max = *v;
        }
    }
    Some(max)
}

pub fn stats_sum(data: &[f64]) -> f64 {
    let mut sum = 0.0;
    for v in data {
        sum += *v;
    }
    sum
}

pub fn stats_mean(data: &[f64]) -> Option<f64> {
    let n = data.len();
    if n == 0 {
        return None;
    }
    Some(stats_sum(data) / (n as f64))
}

fn sum_squared_deltas(data: &[f64]) -> Option<f64> {
    if let Some(mean) = stats_mean(data) {
        let mut ssd = 0.0;
        for v in data {
            let delta = v - mean;
            ssd += delta * delta;
        }
        return Some(ssd);
    }
    None
}

pub fn stats_population_variance(data: &[f64]) -> Option<f64> {
    if let Some(ssd) = sum_squared_deltas(data) {
        let n = data.len() as f64;
        return Some(ssd / n);
    }
    None
}

pub fn stats_sample_variance(data: &[f64]) -> Option<f64> {
    if let Some(ssd) = sum_squared_deltas(data) {
        let nm1 = (data.len() - 1) as f64;
        return Some(ssd / nm1);
    }
    None
}

pub fn stats_population_standard_deviation(data: &[f64]) -> Option<f64> {
    if let Some(spv) = stats_population_variance(data) {
        return Some(f64::sqrt(spv));
    }
    None
}

pub fn stats_sample_standard_deviation(data: &[f64]) -> Option<f64> {
    if let Some(ssv) = stats_sample_variance(data) {
        return Some(f64::sqrt(ssv));
    }
    None
}

pub fn stats_population_skew(data: &[f64]) -> Option<f64> {
    if let Some(mean) = stats_mean(data) {
        let mut sum3 = 0.0;
        for v in data {
            let delta = v - mean;
            sum3 += delta * delta * delta;
        }

        if let Some(ssv) = stats_population_variance(data) {
            let n = data.len() as f64;
            let variance = f64::sqrt(ssv);
            let skew = sum3 / n / (variance * variance * variance);
            return Some(skew);
        }
    }
    None
}

pub fn stats_sample_skew(data: &[f64]) -> Option<f64> {
    if let Some(pop_skew) = stats_population_skew(data) {
        let n = data.len() as f64;
        let skew = f64::sqrt(n * (n - 1.0)) / (n - 2.0) * pop_skew;
        return Some(skew);
    }
    None
}

// The kurtosis functions return _excess_ kurtosis
pub fn stats_population_kurtosis(data: &[f64]) -> Option<f64> {
    if let Some(mean) = stats_mean(data) {
        let n = data.len() as f64;

        let mut sum4 = 0.0;
        for v in data {
            let delta = v - mean;
            sum4 += delta * delta * delta * delta;
        }

        if let Some(variance) = stats_population_variance(data) {
            let kurtosis = sum4 / (variance * variance) / n - 3.0;
            return Some(kurtosis);
        }
    }
    None
}

pub fn stats_sample_kurtosis(data: &[f64]) -> Option<f64> {
    if let Some(population_kurtosis) = stats_population_kurtosis(data) {
        let n = data.len() as f64;
        let kurtosis =
            (n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n + 1.0) * population_kurtosis + 6.0);
        return Some(kurtosis);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // To reduce the file sizes, the unit tests are split into separate files.
    // Private functions are tested here because child modules have access to
    // private data. The public functions are tested in the `./tests` directory.

    #[test]
    fn test_sum_squared_deltas() {
        assert_eq!(sum_squared_deltas(&vec![]), None);
        assert_eq!(sum_squared_deltas(&vec![0.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![1.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![2.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![-1.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![0.0, 0.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![1.0, 1.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![2.0, 2.0]), Some(0.0));
        assert_eq!(sum_squared_deltas(&vec![0.0, 1.0]), Some(0.5));
        assert_eq!(sum_squared_deltas(&vec![1.0, 2.0]), Some(0.5));
        assert_eq!(sum_squared_deltas(&vec![-1.0, 0.0]), Some(0.5));
        assert_eq!(sum_squared_deltas(&vec![-1.0, 0.0, 1.0]), Some(2.0));
    }
}
