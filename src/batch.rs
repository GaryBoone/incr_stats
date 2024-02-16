use crate::error::{Result, StatsError};

//
// Batch functions
//
// These are non-incremental, independent, functions that operate on the data passed into them. They
// are closest to standard textbook formulas for the descriptive statistics. Independence means
// that, unlike the `incr` or `vec` versions, they don't do any calculations that are not required
// for the requested function. For example, the `incr` `update()` function does intermediate moment
// calculations for skewness and kurtosis that are unnecessary if the only statistic later requested
// is mean and variance. It also means that there's no reuse, as in the `vec` versions. For example,
// several functions below require the mean of the data, but each will recalculate it separately.

// Check that the data contains no NaNs, Infs, or -Infs.
pub fn validate(data: &[f64]) -> Result<()> {
    for v in data {
        if f64::is_nan(*v) || f64::is_infinite(*v) {
            return Err(StatsError::InvalidData);
        }
    }
    Ok(())
}

pub fn count(data: &[f64]) -> u32 {
    data.len() as u32
}

pub fn min(data: &[f64]) -> Result<f64> {
    if data.is_empty() {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
}

pub fn max(data: &[f64]) -> Result<f64> {
    if data.is_empty() {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
}

pub fn sum(data: &[f64]) -> Result<f64> {
    if data.is_empty() {
        return Err(StatsError::NotEnoughData);
    }
    Ok(data.iter().fold(0.0, |sum, v| sum + v))
}

pub fn mean(data: &[f64]) -> Result<f64> {
    if data.is_empty() {
        return Err(StatsError::NotEnoughData);
    }
    Ok(sum(data)? / (data.len() as f64))
}

fn sum_squared_deltas(data: &[f64]) -> Result<f64> {
    let mean = mean(data)?;
    let mut ssd = 0.0;
    data.iter().for_each(|v| {
        let delta = v - mean;
        ssd += delta * delta;
    });
    Ok(ssd)
}

// Population variance:
// R: var.pop=function(x){(length(x)-1)/length(x)*var(x)}
// Octave: var(a, 1)
pub fn population_variance(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(sum_squared_deltas(data)? / (data.len() as f64))
}

// Sample variance:
// R: var(a)
// Octave: var(a)
pub fn sample_variance(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    Ok(sum_squared_deltas(data)? / ((data.len() - 1) as f64))
}

// Population standard deviation:
// R: sd.pop=function(x){sd(x)*sqrt((length(x)-1)/length(x))}
// Octave: std(a, 1)
pub fn population_standard_deviation(data: &[f64]) -> Result<f64> {
    Ok(f64::sqrt(population_variance(data)?))
}

// Sample standard deviation:
// R: sd(a)
// Octave: std(a)
pub fn sample_standard_deviation(data: &[f64]) -> Result<f64> {
    Ok(f64::sqrt(sample_variance(data)?))
}

// Population skewness:
// R: library(moments); skewness(a)
// or library(DescTools); Skew(a, method = 1)
// Octave: skewness(a)
pub fn population_skewness(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    let mean = mean(data)?;
    let sum3 = data.iter().fold(0.0, |sum, v| {
        let delta = v - mean;
        sum + delta * delta * delta
    });

    let ssv = population_variance(data)?;
    let n = data.len() as f64;
    let variance = f64::sqrt(ssv);
    if variance == 0.0 {
        return Err(StatsError::Undefined);
    }
    Ok(sum3 / n / (variance * variance * variance))
}

// Sample skewness:
// R: library(DescTools); Skew(a, method=2)
// Octave: skewness(a, 0)
pub fn sample_skewness(data: &[f64]) -> Result<f64> {
    if data.len() <= 2 {
        return Err(StatsError::NotEnoughData);
    }
    let pop_skewness = population_skewness(data)?;
    let n = data.len() as f64;
    let skew = f64::sqrt(n * (n - 1.0)) / (n - 2.0) * pop_skewness;
    Ok(skew)
}

// Population kurtosis:
// The kurtosis functions return _excess_ kurtosis.
//
// Interpretation: kurtosis < 0.0 indicates platykurtic (flat) while kurtosis > 0.0 indicates
// leptokurtic (peaked) and near 0 indicates mesokurtic (normal).
//
// R: library(moments); kurtosis(a) - 3.0 (excess kurtosis)
// or library(DescTools); Kurt(a, method = 1)
// Octave: kurtosis(a) - 3.0
pub fn population_kurtosis(data: &[f64]) -> Result<f64> {
    if data.len() <= 1 {
        return Err(StatsError::NotEnoughData);
    }
    let mean = mean(data)?;
    let n = data.len() as f64;

    let sum4 = data.iter().fold(0.0, |sum4, v| {
        let delta = v - mean;
        sum4 + delta * delta * delta * delta
    });
    let variance = population_variance(data)?;
    if variance == 0.0 {
        return Err(StatsError::Undefined);
    }
    let kurtosis = sum4 / (variance * variance) / n - 3.0;
    Ok(kurtosis)
}

// Sample kurtosis:
// R: library(DescTools); Kurt(a, method = 2)
// Octave: kurtosis(a, 0) - 3.0
pub fn sample_kurtosis(data: &[f64]) -> Result<f64> {
    if data.len() <= 3 {
        return Err(StatsError::NotEnoughData);
    }
    let n = data.len() as f64;
    Ok((n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n + 1.0) * population_kurtosis(data)? + 6.0))
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
