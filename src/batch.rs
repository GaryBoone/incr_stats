use crate::error::{Result, StatsError};

//
//
// Batch functions
//
// These are non-incremental functions that operate only on the data given them.
// They're prefixed with 'Stats'.
//
// TODO: Change stats_* to Stats::*
pub fn stats_count(data: &[f64]) -> u32 {
    data.len() as u32
}

pub fn stats_min(data: &[f64]) -> Result<f64> {
    if data.len() == 0 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
}

pub fn stats_max(data: &[f64]) -> Result<f64> {
    if data.len() == 0 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
}

pub fn stats_sum(data: &[f64]) -> Result<f64> {
    if data.len() == 0 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(0.0, |sum, v| sum + v))
}

pub fn stats_mean(data: &[f64]) -> Result<f64> {
    if data.len() == 0 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(stats_sum(data)? / (data.len() as f64))
}

fn sum_squared_deltas(data: &[f64]) -> Result<f64> {
    let mean = stats_mean(data)?;
    let mut ssd = 0.0;
    data.iter().for_each(|v| {
        let delta = v - mean;
        ssd += delta * delta;
    });
    Ok(ssd)
}

// population_variance:
// R: var.pop=function(x){(length(x)-1)/length(x)*var(x)}
// Oct: var(a, 1)
pub fn stats_population_variance(data: &[f64]) -> Result<f64> {
    // TODO: Check if necessary.
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(sum_squared_deltas(data)? / (data.len() as f64))
}

// sample_variance:
// R: var(a)
// Oct: var(a)
pub fn stats_sample_variance(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(sum_squared_deltas(data)? / ((data.len() - 1) as f64))
}

// population_standard_deviation:
// R: sd.pop=function(x){sd(x)*sqrt((length(x)-1)/length(x))}
// Oct: std(a, 1)
pub fn stats_population_standard_deviation(data: &[f64]) -> Result<f64> {
    Ok(f64::sqrt(stats_population_variance(data)?))
}

// sample_standard_deviation:
// R: sd(a)
// Oct: std(a)
pub fn stats_sample_standard_deviation(data: &[f64]) -> Result<f64> {
    Ok(f64::sqrt(stats_sample_variance(data)?))
}

// population_skew:
// R: skewness(a)
// Oct: skewness(a)
pub fn stats_population_skew(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    let mean = stats_mean(data)?;
    let sum3 = data.iter().fold(0.0, |sum, v| {
        let delta = v - mean;
        sum + delta * delta * delta
    });

    let ssv = stats_population_variance(data)?;
    let n = data.len() as f64;
    let variance = f64::sqrt(ssv);
    if variance == 0.0 {
        return Err(StatsError::Undefined);
    }
    Ok(sum3 / n / (variance * variance * variance))
}

// sample_skew:
// R: Skew(a, method=2) => -0.5656994001961358 (G_1 = g_1 * sqrt(n(n-1)) / (n-2))
// Oct: skewness(a10, 0)
pub fn stats_sample_skew(data: &[f64]) -> Result<f64> {
    if data.len() <= 2 {
        return Err(StatsError::NotEnoughData);
    }
    let pop_skew = stats_population_skew(data)?;
    let n = data.len() as f64;
    let skew = f64::sqrt(n * (n - 1.0)) / (n - 2.0) * pop_skew;
    Ok(skew)
}

// The kurtosis functions return _excess_ kurtosis
// population_kurtosis:
// R: kurtosis(a)-3.0 (ie excess kurtosis), or Kurt(a10, method = 1)
// Oct: kurtosis(a)-3.0
pub fn stats_population_kurtosis(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    let mean = stats_mean(data)?;
    let n = data.len() as f64;

    let sum4 = data.iter().fold(0.0, |sum4, v| {
        let delta = v - mean;
        sum4 + delta * delta * delta * delta
    });
    let variance = stats_population_variance(data)?;
    if variance == 0.0 {
        return Err(StatsError::Undefined);
    }
    let kurtosis = sum4 / (variance * variance) / n - 3.0;
    Ok(kurtosis)
}

// sample_kurtosis:
// R: Kurt(a, method = 2)
// Oct: kurtosis(a, 0)-3.0
pub fn stats_sample_kurtosis(data: &[f64]) -> Result<f64> {
    if data.len() <= 3 {
        return Err(StatsError::NotEnoughData);
    }
    let n = data.len() as f64;
    Ok((n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n + 1.0) * stats_population_kurtosis(data)? + 6.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    // To reduce the file sizes, the unit tests are split into separate files.
    // Private functions are tested here because child modules have access to
    // private data. The public functions are tested in the `./tests` directory.

    #[test]
    fn test_sum_squared_deltas() {
        assert_eq!(sum_squared_deltas(&vec![]), Err(StatsError::NotEnoughData));
        assert_eq!(sum_squared_deltas(&vec![0.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![1.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![2.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![-1.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![0.0, 0.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![1.0, 1.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![2.0, 2.0]), Ok(0.0));
        assert_eq!(sum_squared_deltas(&vec![0.0, 1.0]), Ok(0.5));
        assert_eq!(sum_squared_deltas(&vec![1.0, 2.0]), Ok(0.5));
        assert_eq!(sum_squared_deltas(&vec![-1.0, 0.0]), Ok(0.5));
        assert_eq!(sum_squared_deltas(&vec![-1.0, 0.0, 1.0]), Ok(2.0));
    }
}
