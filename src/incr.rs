use crate::error::{Result, StatsError};

#[derive(Default, Clone, Debug)]
pub struct Stats {
    n_int: u32, // Maintain the size as an int to avoid frequent casting.
    n: f64,
    min: f64,
    max: f64,
    sum: f64,
    mean: f64,
    m2: f64,
    m3: f64,
    m4: f64,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            ..Default::default()
        }
    }

    // Update the moments with the given value.
    pub fn update(&mut self, x: f64) -> Result<()> {
        if f64::is_nan(x) || f64::is_infinite(x) {
            return Err(StatsError::InvalidData);
        }
        if self.n_int == 0 || x < self.min {
            self.min = x
        }
        if self.n_int == 0 || x > self.max {
            self.max = x
        }
        // Perform incremental updates from the previous values. The updates are done in careful
        // order; the values used are the prior values until they are updated.
        self.sum += x;
        let n_ = self.n; // Prior  n.
        self.n_int += 1;
        self.n += 1.0;
        let delta = x - self.mean; // Deviation from the prior mean.
        let delta_n = delta / self.n;
        let delta_n2 = delta_n * delta_n;
        let term1 = delta * delta_n * n_;
        // Fourth moment, used to calculate kurtosis.
        self.m4 += term1 * delta_n2 * (self.n * self.n - 3.0 * self.n + 3.0)
            + 6.0 * delta_n2 * self.m2
            - 4.0 * delta_n * self.m3;
        // Third moment, used to calculate skewness.
        self.m3 += term1 * delta_n * (self.n - 2.0) - 3.0 * delta_n * self.m2;
        // Second moment, used to calculate variance.
        self.m2 += term1;
        // First moment, the mean.
        self.mean += delta_n;

        Ok(())
    }

    pub fn count(&self) -> u32 {
        self.n_int
    }

    pub fn min(&self) -> Result<f64> {
        if self.n_int == 0 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.min)
    }

    pub fn max(&self) -> Result<f64> {
        if self.n_int == 0 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.max)
    }

    pub fn sum(&self) -> Result<f64> {
        if self.n_int == 0 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.sum)
    }

    pub fn mean(&self) -> Result<f64> {
        if self.n_int == 0 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.mean)
    }

    // Update the stats with the given array of values using incremental updates for each value. If
    // all of the data is contained in a single array, the batch functions below would be faster.
    // However, this function allows incremental updates with more than one value at a time.
    pub fn array_update(&mut self, data: &[f64]) -> Result<()> {
        for v in data {
            self.update(*v)?;
        }
        Ok(())
    }

    // Population variance:
    // R: var.pop=function(x){(length(x)-1)/length(x)*var(x)}
    // Octave: var(a, 1)
    pub fn population_variance(&self) -> Result<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.m2 / self.n)
    }

    // Sample variance:
    // R: var(a)
    // Octave: var(a)
    pub fn sample_variance(&self) -> Result<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(self.m2 / (self.n - 1.0))
    }

    // Population standard deviation:
    // R: sd.pop=function(x){sd(x)*sqrt((length(x)-1)/length(x))}
    // Octave: std(a, 1)
    pub fn population_standard_deviation(&self) -> Result<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(f64::sqrt(self.population_variance()?))
    }

    // Sample standard deviation:
    // R: sd(a)
    // Octave: std(a)
    pub fn sample_standard_deviation(&self) -> Result<f64> {
        if self.n_int <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(f64::sqrt(self.sample_variance()?))
    }

    // Population skewness:
    // R: library(moments); skewness(a)
    // or library(DescTools); Skew(a, method = 1)
    // Octave: skewness(a)
    pub fn population_skewness(&self) -> Result<f64> {
        if self.n_int <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        if self.m2 == 0.0 {
            return Err(StatsError::Undefined);
        }
        Ok(f64::sqrt(self.n / (self.m2 * self.m2 * self.m2)) * self.m3)
    }

    // Sample skewness:
    // R: library(DescTools); Skew(a, method=2)
    // Octave: skewness(a, 0)
    pub fn sample_skewness(&self) -> Result<f64> {
        if self.n_int <= 2 {
            return Err(StatsError::NotEnoughData);
        }
        Ok(f64::sqrt(self.n * (self.n - 1.0)) / (self.n - 2.0) * self.population_skewness()?)
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
    pub fn population_kurtosis(&self) -> Result<f64> {
        if self.n_int <= 1 {
            return Err(StatsError::NotEnoughData);
        }
        if self.m2 == 0.0 {
            return Err(StatsError::Undefined);
        }
        let k = (self.n * self.m4) / (self.m2 * self.m2) - 3.0;
        Ok(k)
    }

    // Sample kurtosis:
    // R: library(DescTools); Kurt(a, method = 2)
    // Octave: kurtosis(a, 0) - 3.0
    pub fn sample_kurtosis(&self) -> Result<f64> {
        if self.n_int <= 3 {
            return Err(StatsError::NotEnoughData);
        }
        let k = self.population_kurtosis()?;
        Ok((self.n - 1.0) / ((self.n - 2.0) * (self.n - 3.0)) * ((self.n + 1.0) * k + 6.0))
    }
}
