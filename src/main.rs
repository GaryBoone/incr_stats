#[derive(Default)]
struct Stats {
    n_int: u32,
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
    fn new() -> Self {
        Stats {
            ..Default::default()
        }
    }

    // Update the stats with the given value.
    fn update(&mut self, x: f64) {
        if self.n == 0. || x < self.min {
            self.min = x
        }
        if self.n == 0.0 || x > self.max {
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

    fn count(&self) -> u32 {
        self.n_int
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }

    fn sum(&self) -> f64 {
        self.sum
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    // Update the stats with the given array of values using incremental updates for each value. If
    // all of the data is contained in a single array, the batch functions below would be faster.
    // However, this function allows incremental updates when the data is given in groups.
    fn update_array(&mut self, data: &[f64]) {
        for v in data {
            self.update(*v);
        }
    }

    fn population_variance(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        Some(self.m2 / self.n)
    }

    fn sample_variance(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        Some(self.m2 / (self.n - 1.0))
    }

    fn population_standard_deviation(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        match self.population_variance() {
            None => None,
            Some(pv) => Some(f64::sqrt(pv)),
        }
    }

    fn sample_standard_deviation(&self) -> Option<f64> {
        if self.n_int == 0 || self.n_int == 1 {
            return None;
        }
        match self.sample_variance() {
            None => None,
            Some(sv) => Some(f64::sqrt(sv)),
        }
    }

    fn population_skew(&self) -> Option<f64> {
        let k = f64::sqrt(self.n / (self.m2 * self.m2 * self.m2)) * self.m3;
        if k.is_nan() {
            None
        } else {
            Some(k)
        }
    }

    fn sample_skew(&self) -> Option<f64> {
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
    fn population_kurtosis(&self) -> Option<f64> {
        let k = (self.n * self.m4) / (self.m2 * self.m2) - 3.0;
        if k.is_nan() {
            None
        } else {
            Some(k)
        }
    }

    fn sample_kurtosis(&self) -> Option<f64> {
        if self.n_int == 2 || self.n_int == 3 {
            return None;
        }
        match self.population_kurtosis() {
            None => None,
            Some(k) => Some(
                (self.n - 1.0) / ((self.n - 2.0) * (self.n - 3.0)) * ((self.n + 1.0) * k + 6.0),
            ),
        }
    }
}

//
//
// Batch functions
//
// These are non-incremental functions that operate only on the data given them.
// They're prefixed with 'Stats'.
//
fn stats_count(data: &[f64]) -> u32 {
    data.len() as u32
}

// func StatsMin(data []float64) float64 {
// 	if len(data) == 0 {
// 		return math.NaN()
// 	}
// 	min := data[0]
// 	for _, v := range data {
// 		if v < min {
// 			min = v
// 		}
// 	}
// 	return min
// }

// func StatsMax(data []float64) float64 {
// 	if len(data) == 0 {
// 		return math.NaN()
// 	}
// 	max := data[0]
// 	for _, v := range data {
// 		if v > max {
// 			max = v
// 		}
// 	}
// 	return max
// }

// func StatsSum(data []float64) (sum float64) {
// 	for _, v := range data {
// 		sum += v
// 	}
// 	return
// }

// func StatsMean(data []float64) float64 {
// 	return StatsSum(data) / float64(len(data))
// }

// func sumSquaredDeltas(data []float64) (ssd float64) {
// 	mean := StatsMean(data)
// 	for _, v := range data {
// 		delta := v - mean
// 		ssd += delta * delta
// 	}
// 	return
// }

// func StatsPopulationVariance(data []float64) float64 {
// 	n := float64(len(data))
// 	ssd := sumSquaredDeltas(data)
// 	return ssd / n
// }

// func StatsSampleVariance(data []float64) float64 {
// 	n := float64(len(data))
// 	ssd := sumSquaredDeltas(data)
// 	return ssd / (n - 1.0)
// }

// func StatsPopulationStandardDeviation(data []float64) float64 {
// 	return math.Sqrt(StatsPopulationVariance(data))
// }

// func StatsSampleStandardDeviation(data []float64) float64 {
// 	return math.Sqrt(StatsSampleVariance(data))
// }

// func StatsPopulationSkew(data []float64) (skew float64) {
// 	mean := StatsMean(data)
// 	n := float64(len(data))

// 	sum3 := 0.0
// 	for _, v := range data {
// 		delta := v - mean
// 		sum3 += delta * delta * delta
// 	}

// 	variance := math.Sqrt(StatsPopulationVariance(data))
// 	skew = sum3 / n / (variance * variance * variance)
// 	return
// }

// func StatsSampleSkew(data []float64) float64 {
// 	popSkew := StatsPopulationSkew(data)
// 	n := float64(len(data))
// 	return math.Sqrt(n*(n-1.0)) / (n - 2.0) * popSkew
// }

// // The kurtosis functions return _excess_ kurtosis
// func StatsPopulationKurtosis(data []float64) (kurtosis float64) {
// 	mean := StatsMean(data)
// 	n := float64(len(data))

// 	sum4 := 0.0
// 	for _, v := range data {
// 		delta := v - mean
// 		sum4 += delta * delta * delta * delta
// 	}

// 	variance := StatsPopulationVariance(data)
// 	kurtosis = sum4/(variance*variance)/n - 3.0
// 	return
// }

// func StatsSampleKurtosis(data []float64) float64 {
// 	populationKurtosis := StatsPopulationKurtosis(data)
// 	n := float64(len(data))
// 	return (n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n+1.0)*populationKurtosis + 6.0)
// }

fn main() {
    println!("Hello, world!");
    let mut s = Stats::new();
    s.update(1.);
}

#[cfg(test)]
mod tests {
    mod check;
    mod test_array;
    mod test_batch;
    mod test_inc;
}
