use crate::error::StatsError;

#[derive(Default)]
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

    // Update the stats with the given value.
    pub fn update(&mut self, x: f64) {
        if self.n_int == 0 || x < self.min {
            self.min = x
        }
        if self.n_int == 0 || x > self.max {
            self.max = x
        }
        self.sum += x;
        let n_minus_1 = self.n;
        self.n_int += 1;
        self.n += 1.0;
        let delta = x - self.mean;
        let delta_n = delta / self.n;
        let delta_n2 = delta_n * delta_n;
        let term1 = delta * delta_n * n_minus_1;
        self.mean += delta_n;
        self.m4 += term1 * delta_n2 * (self.n * self.n - 3.0 * self.n + 3.0)
            + 6.0 * delta_n2 * self.m2
            - 4.0 * delta_n * self.m3;
        self.m3 += term1 * delta_n * (self.n - 2.0) - 3.0 * delta_n * self.m2;
        self.m2 += term1;
    }

    pub fn count(&self) -> u32 {
        self.n_int
    }

    pub fn min(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 {
            return Err(StatsError::NoData);
        }
        Ok(self.min)
    }

    pub fn max(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 {
            return Err(StatsError::NoData);
        }
        Ok(self.max)
    }

    pub fn sum(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 {
            return Err(StatsError::NoData);
        }
        Ok(self.sum)
    }

    pub fn mean(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 {
            return Err(StatsError::NoData);
        }
        Ok(self.mean)
    }

    // Update the stats with the given array of values using incremental updates for each value. If
    // all of the data is contained in a single array, the batch functions below would be faster.
    // However, this function allows incremental updates with more than one value at a time.
    pub fn update_array(&mut self, data: &[f64]) {
        for v in data {
            self.update(*v);
        }
    }

    pub fn population_variance(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 {
            return Err(StatsError::NoData);
        }
        if self.n_int == 1 {
            return Err(StatsError::SecondMomentUndefined);
        }
        Ok(self.m2 / self.n)
    }

    pub fn sample_variance(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        Some(self.m2 / (self.n - 1.0))
    }

    pub fn population_standard_deviation(&self) -> Result<f64, StatsError> {
        if self.n_int == 0 || self.n_int == 1 {
            return Err(StatsError::SecondMomentUndefined);
        }
        Ok(f64::sqrt(self.population_variance()?))
    }

    pub fn sample_standard_deviation(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        match self.sample_variance() {
            None => None,
            Some(sv) => Some(f64::sqrt(sv)),
        }
    }

    pub fn population_skew(&self) -> Option<f64> {
        let k = f64::sqrt(self.n / (self.m2 * self.m2 * self.m2)) * self.m3;
        if k.is_nan() {
            None
        } else {
            Some(k)
        }
    }

    pub fn sample_skew(&self) -> Option<f64> {
        if self.n_int == 2 {
            return None;
        }
        match self.population_skew() {
            None => None,
            Some(s) => Some(f64::sqrt(self.n * (self.n - 1.0)) / (self.n - 2.0) * s),
        }
    }

    // The kurtosis functions return _excess_ kurtosis, so that the kurtosis of a normal
    // distribution = 0.0. Then kurtosis < 0.0 indicates platykurtic (flat) while
    // kurtosis > 0.0 indicates leptokurtic (peaked) and near 0 indicates mesokurtic.Update
    pub fn population_kurtosis(&self) -> Option<f64> {
        let k = (self.n * self.m4) / (self.m2 * self.m2) - 3.0;
        if k.is_nan() {
            None
        } else {
            Some(k)
        }
    }

    pub fn sample_kurtosis(&self) -> Result<f64, StatsError> {
        if self.n_int == 2 {
            return Err(StatsError::SecondMomentUndefined);
        }
        if self.n_int == 3 {
            return Err(StatsError::ThirdMomentUndefined);
        }
        match self.population_kurtosis() {
            None => Err(StatsError::FourthMomentUndefined),
            Some(k) => {
                Ok((self.n - 1.0) / ((self.n - 2.0) * (self.n - 3.0)) * ((self.n + 1.0) * k + 6.0))
            }
        }
    }
}
