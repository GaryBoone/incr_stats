use crate::batch;
use crate::error::{Result, StatsError};

// This module provides optimized stored-array functions that efficiently
// calculate all of the descriptive statistics. Efficiency is gained by taking
// advantage of the fact that the higher order statistical moments depend on
// lower ones, so reuse is possible. For example, kurtosis depends on variance
// which depends on the mean.

#[derive(Default, Debug, PartialEq)]
pub struct Stats<'a> {
    data: &'a [f64],
    min: Option<f64>,
    max: Option<f64>,
    sum: Option<f64>,
    mean: Option<f64>,
    population_variance: Option<f64>,
    sum_squared_deltas: Option<f64>,
    sample_variance: Option<f64>,
    population_standard_deviation: Option<f64>,
    sample_standard_deviation: Option<f64>,
    population_skewness: Option<f64>,
    sample_skewness: Option<f64>,
    population_kurtosis: Option<f64>,
    sample_kurtosis: Option<f64>,
}

impl<'a> Stats<'a> {
    pub fn new(data: &'a [f64]) -> Result<Self> {
        batch::validate(data)?;
        Ok(Stats {
            data,
            ..Default::default()
        })
    }

    pub fn count(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn min(&mut self) -> Result<f64> {
        if let Some(min) = self.min {
            return Ok(min);
        }
        let min = batch::min(self.data)?;
        self.min = Some(min);
        Ok(min)
    }

    pub fn max(&mut self) -> Result<f64> {
        if let Some(max) = self.max {
            return Ok(max);
        }
        let max = batch::max(self.data)?;
        self.max = Some(max);
        Ok(max)
    }

    pub fn sum(&mut self) -> Result<f64> {
        if let Some(sum) = self.sum {
            return Ok(sum);
        }
        let sum = batch::sum(self.data)?;
        self.sum = Some(sum);
        Ok(sum)
    }
    pub fn mean(&mut self) -> Result<f64> {
        if let Some(mean) = self.mean {
            return Ok(mean);
        }
        if self.data.is_empty() {
            return Err(StatsError::NotEnoughData);
        }
        let mean = self.sum()? / (self.data.len() as f64);
        self.mean = Some(mean);
        Ok(mean)
    }

    fn sum_squared_deltas(&mut self) -> Result<f64> {
        if let Some(sum_squared_deltas) = self.sum_squared_deltas {
            return Ok(sum_squared_deltas);
        }
        let mean = self.mean()?;
        let mut ssd = 0.0;
        self.data.iter().for_each(|v| {
            let delta = v - mean;
            ssd += delta * delta;
        });
        self.sum_squared_deltas = Some(ssd);
        Ok(ssd)
    }

    // Population variance:
    // R: var.pop=function(x){(length(x)-1)/length(x)*var(x)}
    // Octave: var(a, 1)
    pub fn population_variance(&mut self) -> Result<f64> {
        if let Some(population_variance) = self.population_variance {
            return Ok(population_variance);
        }
        if self.data.len() <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        let population_variance = self.sum_squared_deltas()? / (self.data.len() as f64);
        self.population_variance = Some(population_variance);
        Ok(population_variance)
    }

    // Sample variance:
    // R: var(a)
    // Octave: var(a)
    pub fn sample_variance(&mut self) -> Result<f64> {
        if let Some(sample_variance) = self.sample_variance {
            return Ok(sample_variance);
        }
        if self.data.len() <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        let sample_variance = self.sum_squared_deltas()? / ((self.data.len() - 1) as f64);
        self.sample_variance = Some(sample_variance);
        Ok(sample_variance)
    }

    // Population standard deviation:
    // R: sd.pop=function(x){sd(x)*sqrt((length(x)-1)/length(x))}
    // Octave: std(a, 1)
    pub fn population_standard_deviation(&mut self) -> Result<f64> {
        if let Some(population_standard_deviation) = self.population_standard_deviation {
            return Ok(population_standard_deviation);
        }
        let population_standard_deviation = f64::sqrt(self.population_variance()?);
        self.population_standard_deviation = Some(population_standard_deviation);
        Ok(population_standard_deviation)
    }

    // Sample standard deviation:
    // R: sd(a)
    // Octave: std(a)
    pub fn sample_standard_deviation(&mut self) -> Result<f64> {
        if let Some(sample_standard_deviation) = self.sample_standard_deviation {
            return Ok(sample_standard_deviation);
        }
        let sample_standard_deviation = f64::sqrt(self.sample_variance()?);
        self.sample_standard_deviation = Some(sample_standard_deviation);
        Ok(sample_standard_deviation)
    }

    // Population skewness:
    // R: library(moments); skewness(a)
    // or library(DescTools); Skew(a, method = 1)
    // Octave: skewness(a)
    pub fn population_skewness(&mut self) -> Result<f64> {
        if let Some(population_skewness) = self.population_skewness {
            return Ok(population_skewness);
        }
        if self.data.len() <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        let mean = self.mean()?;
        let sum3 = self.data.iter().fold(0.0, |sum, v| {
            let delta = v - mean;
            sum + delta * delta * delta
        });

        let ssv = self.population_variance()?;
        let n = self.data.len() as f64;
        let variance = f64::sqrt(ssv);
        if variance == 0.0 {
            return Err(StatsError::Undefined);
        }
        let population_skewness = sum3 / n / (variance * variance * variance);
        self.population_skewness = Some(population_skewness);
        Ok(population_skewness)
    }

    // Sample skewness:
    // R: library(DescTools); Skew(a, method=2)
    // Octave: skewness(a, 0)
    pub fn sample_skewness(&mut self) -> Result<f64> {
        if let Some(sample_skewness) = self.sample_skewness {
            return Ok(sample_skewness);
        }
        if self.data.len() <= 2 {
            return Err(StatsError::NotEnoughData);
        }
        let pop_skewness = self.population_skewness()?;
        let n = self.data.len() as f64;
        let sample_skewness = f64::sqrt(n * (n - 1.0)) / (n - 2.0) * pop_skewness;
        self.sample_skewness = Some(sample_skewness);
        Ok(sample_skewness)
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
    pub fn population_kurtosis(&mut self) -> Result<f64> {
        if let Some(population_kurtosis) = self.population_kurtosis {
            return Ok(population_kurtosis);
        }
        if self.data.len() <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        let mean = self.mean()?;
        let n = self.data.len() as f64;

        let sum4 = self.data.iter().fold(0.0, |sum4, v| {
            let delta = v - mean;
            sum4 + delta * delta * delta * delta
        });
        let variance = self.population_variance()?;
        if variance == 0.0 {
            return Err(StatsError::Undefined);
        }
        let population_kurtosis = sum4 / (variance * variance) / n - 3.0;
        self.population_kurtosis = Some(population_kurtosis);
        Ok(population_kurtosis)
    }

    // Sample kurtosis:
    // R: library(DescTools); Kurt(a, method = 2)
    // Octave: kurtosis(a, 0) - 3.0
    pub fn sample_kurtosis(&mut self) -> Result<f64> {
        if let Some(sample_kurtosis) = self.sample_kurtosis {
            return Ok(sample_kurtosis);
        }
        if self.data.len() <= 3 {
            return Err(StatsError::NotEnoughData);
        }
        let n = self.data.len() as f64;
        let sample_kurtosis =
            (n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n + 1.0) * self.population_kurtosis()? + 6.0);
        self.sample_kurtosis = Some(sample_kurtosis);
        Ok(sample_kurtosis)
    }
}

pub fn descriptive(a: &[f64]) -> Result<Stats> {
    let mut d = Stats::new(a)?;
    d.min()?;
    d.max()?;
    d.sum()?;
    d.mean()?;
    d.population_variance()?;
    d.sample_variance()?;
    d.population_standard_deviation()?;
    d.sample_standard_deviation()?;
    d.population_skewness()?;
    d.sample_skewness()?;
    d.population_kurtosis()?;
    d.sample_kurtosis()?;
    Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;

    // To reduce the file sizes, the unit tests are split into separate files.
    // Private functions are tested here because child modules have access to
    // private data. The public functions are tested in the `./tests` directory.

    #[test]
    fn test_sum_squared_deltas() {
        assert_eq!(
            Stats::new(&vec![]).unwrap().sum_squared_deltas(),
            Err(StatsError::NotEnoughData)
        );
        assert_eq!(
            Stats::new(&vec![0.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![1.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![2.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![-1.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![0.0, 0.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![1.0, 1.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![2.0, 2.0]).unwrap().sum_squared_deltas(),
            Ok(0.0)
        );
        assert_eq!(
            Stats::new(&vec![0.0, 1.0]).unwrap().sum_squared_deltas(),
            Ok(0.5)
        );
        assert_eq!(
            Stats::new(&vec![1.0, 2.0]).unwrap().sum_squared_deltas(),
            Ok(0.5)
        );
        assert_eq!(
            Stats::new(&vec![-1.0, 0.0]).unwrap().sum_squared_deltas(),
            Ok(0.5)
        );
        assert_eq!(
            Stats::new(&vec![-1.0, 0.0, 1.0])
                .unwrap()
                .sum_squared_deltas(),
            Ok(2.0)
        );
    }
}
