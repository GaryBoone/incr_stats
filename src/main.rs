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

    // // Update the stats with the given array of values.
    // func (d *Stats) UpdateArray(data []float64) {
    // 	for _, v := range data {
    // 		d.Update(v)
    // 	}

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

// //
// //
// // Batch functions
// //
// // These are non-incremental functions that operate only on the data given them.
// // They're prefixed with 'Calc'.
// //
// func StatsCount(data []float64) int {
// 	return len(data)
// }

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
    mod tests;

    use super::*;
    use tests::Checker;

    #[test]
    fn test_update() {
        let mut d = Stats::new();
        d.update(2.3);
        chk!(d.count(), 1u32);
        chk!(d.min(), 2.3);
        chk!(d.max(), 2.3);
        chk!(d.sum(), 2.3);
        chk!(d.mean(), 2.3);
        chk!(d.population_variance(), None);
        chk!(d.sample_variance(), None);
        chk!(d.population_standard_deviation(), None);
        chk!(d.sample_standard_deviation(), None);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 2 values
    fn test_update2() {
        let mut d = Stats::new();
        d.update(2.3);
        d.update(0.4);
        chk!(d.count(), 2u32);
        chk!(d.min(), 0.4);
        chk!(d.max(), 2.3);
        chk!(d.sum(), 2.7);
        chk!(d.mean(), 1.35);
        chk!(d.population_variance().unwrap(), 0.9025);
        chk!(d.sample_variance().unwrap(), 1.805);
        chk!(d.population_standard_deviation().unwrap(), 0.95);
        chk!(d.sample_standard_deviation().unwrap(), 1.34350288425444);
        chk!(d.population_skew().unwrap(), 0.0);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis().unwrap(), -2.0);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 3 values.
    fn test_update3() {
        let mut d = Stats::new();
        d.update(2.3);
        d.update(0.4);
        d.update(-3.4);
        chk!(d.count(), 3u32);
        chk!(d.min(), -3.4);
        chk!(d.max(), 2.3);
        chk!(d.sum(), -0.7);
        chk!(d.mean(), -0.2333333333333334);
        chk!(d.population_variance().unwrap(), 5.615555555555554);
        chk!(d.sample_variance().unwrap(), 8.42333333333333);
        chk!(d.population_standard_deviation().unwrap(), 2.36971634495683);
        chk!(d.sample_standard_deviation().unwrap(), 2.90229794013870);
        chk!(d.population_skew().unwrap(), -0.3818017741606063);
        chk!(d.sample_skew(), Some(-0.9352195295828242));
        chk!(d.population_kurtosis().unwrap(), -1.5);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 4 values. Now all of the statistics are available.
    fn test_update4() {
        let mut d = Stats::new();

        d.update(1.0);
        d.update(2.0);
        d.update(3.0);
        d.update(4.0);
        chk!(d.count(), 4u32);
        chk!(d.min(), 1.0);
        chk!(d.max(), 4.0);
        chk!(d.sum(), 10.0);
        chk!(d.mean(), 2.5);
        chk!(d.population_variance().unwrap(), 1.25);
        chk!(d.sample_variance().unwrap(), 1.666666666666667);
        chk!(
            d.population_standard_deviation().unwrap(),
            1.118033988749895
        );
        chk!(d.sample_standard_deviation().unwrap(), 1.290994448735806);
        chk!(d.population_skew().unwrap(), 0.0);
        chk!(d.sample_skew().unwrap(), 0.0);
        chk!(d.population_kurtosis().unwrap(), -1.36);
        chk!(d.sample_kurtosis().unwrap(), -1.2);
    }

    #[test]
    // Update() 5 values.
    fn test_update5() {
        let mut d = Stats::new();

        d.update(1.0);
        d.update(2.0);
        d.update(3.0);
        d.update(4.0);
        d.update(5.0);
        chk!(d.count(), 5u32);
        chk!(d.min(), 1.0);
        chk!(d.max(), 5.0);
        chk!(d.sum(), 15.0);
        chk!(d.mean(), 3.0);
        chk!(d.population_variance().unwrap(), 2.0);
        chk!(d.sample_variance().unwrap(), 2.5);
        chk!(
            d.population_standard_deviation().unwrap(),
            1.414213562373095
        );
        chk!(d.sample_standard_deviation().unwrap(), 1.5811388300841898);
        chk!(d.population_skew().unwrap(), 0.0);
        chk!(d.sample_skew().unwrap(), 0.0);
        chk!(d.population_kurtosis().unwrap(), -1.3);
        chk!(d.sample_kurtosis().unwrap(), -1.2);
    }

    #[test]
    // Update() 10 values.
    fn test_update10() {
        let mut _d = Stats::new();

        // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
        // 	for _, v := range a {
        // 		d.Update(v)
        // 	}
        // chk!(d.count(), 10);
        // check_f64(d.min(), -123.4, "min");
        // check_f64(d.max(), 115.0, "max");
        // check_f64(d.sum(), 62.83, "sum");
        // check_f64(d.mean(), 6.283, "mean");
        // check_f64(
        //     d.population_variance().unwrap(),
        //     3165.19316100,
        //     "population_variance",
        // );
        // check_f64(d.sample_variance(), 3516.88129, "sample_variance");
        // check_f64(
        //     d.population_standard_deviation(),
        //     56.2600494223032,
        //     "population_standard_deviation",
        // );
        // check_f64(
        //     d.sample_standard_deviation(),
        //     59.3032991493728,
        //     "sample_standard_deviation",
        // );
        // check_f64(d.population_skew().unwrap(), -0.4770396201629045, "population_skew");
        // check_f64(d.sample_skew().unwrap(), -0.565699400196136, "sample_skew");
        // check_f64(
        //     d.population_kurtosis().unwrap(),
        //     1.253240236214162,
        //     "population_kurtosis",
        // );
        // check_f64(d.sample_kurtosis(), 3.179835417592894, "sample_kurtosis");
    }

    #[test]
    // Update by array. In this case, we use slices to update via half of the array at a time.
    fn test_update_array10() {
        let mut _d = Stats::new();

        // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
        // 	// load the first half of the array
        // 	d.UpdateArray(a[:5])
        // chk!(d.count(), 5);
        // check_f64(d.min(), -2.0, "min");
        // check_f64(d.max(), 115.0, "max");
        // check_f64(d.sum(), 174, "sum");
        // check_f64(d.mean(), 34.8, "mean");
        // check_f64(d.population_variance().unwrap(), 1910.56, "population_variance");
        // check_f64(d.sample_variance(), 2388.2, "sample_variance");
        // check_f64(
        //     d.population_standard_deviation(),
        //     43.70995309995196,
        //     "population_standard_deviation",
        // );
        // check_f64(
        //     d.sample_standard_deviation(),
        //     48.8692132124101,
        //     "sample_standard_deviation",
        // );
        // check_f64(d.population_skew().unwrap(), 1.003118841855798, "population_skew");
        // check_f64(d.sample_skew().unwrap(), 1.495361279933617, "sample_skew");
        // check_f64(
        //     d.population_kurtosis().unwrap(),
        //     -0.5476524250400354,
        //     "population_kurtosis",
        // );
        // check_f64(d.sample_kurtosis(), 1.809390299839858, "sample_kurtosis");

        // 	// load rest of array. The results will be cumulative.
        // 	d.UpdateArray(a[5:])
        // chk!(d.count(), 10);
        // check_f64(d.min(), -123.4, "min");
        // check_f64(d.max(), 115.0, "max");
        // check_f64(d.sum(), 62.83, "sum");
        // check_f64(d.mean(), 6.283, "mean");
        // check_f64(
        //     d.population_variance().unwrap(),
        //     3165.19316100,
        //     "population_variance",
        // );
        // check_f64(d.sample_variance(), 3516.88129, "sample_variance");
        // check_f64(
        //     d.population_standard_deviation(),
        //     56.2600494223032,
        //     "population_standard_deviation",
        // );
        // check_f64(
        //     d.sample_standard_deviation(),
        //     59.3032991493728,
        //     "sample_standard_deviation",
        // );
        // check_f64(d.population_skew().unwrap(), -0.4770396201629045, "population_skew");
        // check_f64(d.sample_skew().unwrap(), -0.565699400196136, "sample_skew");
        // check_f64(
        //     d.population_kurtosis().unwrap(),
        //     1.253240236214162,
        //     "population_kurtosis",
        // );
        // check_f64(d.sample_kurtosis(), 3.179835417592894, "sample_kurtosis");
    }

    // Test the batch functions. Calculate the descriptive stats on the whole array.
    // func TestArrayStats(t *testing.T) {
    // 	a := []float64{1.0, 2.0, 3.0, 4.0, 5.0}
    // chk!(StatsCount(a), 5);
    // check_f64(StatsMin(a), 1.0, "min");
    // check_f64(StatsMax(a), 5.0, "max");
    // check_f64(StatsSum(a), 15.0, "sum");
    // check_f64(StatsMean(a), 3.0, "mean");
    // check_f64(StatsPopulationVariance(a), 2.0, "population_variance");
    // check_f64(StatsSampleVariance(a), 2.5, "sample_variance");
    // check_f64(StatsPopulationStandardDeviation(a), 1.414213562373095, "population_standard_deviation");
    // check_f64(StatsSampleStandardDeviation(a), 1.5811388300841898, "sample_standard_deviation");
    // check_f64(StatsPopulationSkew(a), 0.0, "population_skew");
    // check_f64(StatsSampleSkew(a), 0.0, "sample_skew");
    // check_f64(StatsPopulationKurtosis(a), -1.3, "population_kurtosis");
    // check_f64(StatsSampleKurtosis(a), -1.2, "sample_kurtosis");
    // }

    // func TestArrayStats2(t *testing.T) {
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // chk!(StatsCount(a), 10);
    // check_f64(StatsMin(a), -123.4, "min");
    // check_f64(StatsMax(a), 115.0, "max");
    // check_f64(StatsSum(a), 62.83, "sum");
    // check_f64(StatsMean(a), 6.283, "mean");
    // check_f64(StatsPopulationVariance(a), 3165.19316100, "population_variance");
    // check_f64(StatsSampleVariance(a), 3516.88129, "sample_variance");
    // check_f64(StatsPopulationStandardDeviation(a), 56.2600494223032, "population_standard_deviation");
    // check_f64(StatsSampleStandardDeviation(a), 59.3032991493728, "sample_standard_deviation");
    // check_f64(StatsPopulationSkew(a), -0.4770396201629045, "population_skew");
    // check_f64(StatsSampleSkew(a), -0.565699400196136, "sample_skew");
    // check_f64(StatsPopulationKurtosis(a), 1.253240236214162, "population_kurtosis");
    // check_f64(StatsSampleKurtosis(a), 3.179835417592894, "sample_kurtosis");
    // }

    // //
    // //
    // // Benchmark tests
    // //
    // // run with: go test stats.go stats_test.go -bench="Benchmark"
    // //

    // func BenchmarkUpdate(b *testing.B) {
    // let mut d = Stats::new();
    // 	for i := 0; i < b.N; i++ {
    // 		d.Update(3.5)
    // 	}
    // }

    // // Test the incremental Variance function by itself. This result is how fast the
    // // Variance is calculated not including the time to incrementally update the Stats
    // // structure with 10 values.
    // func BenchmarkPopulationVariance10(b *testing.B) {
    // 	b.StopTimer()
    //  let mut d = Stats::new();
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for _, v := range a {
    // 		d.Update(v)
    // 	}
    // 	b.StartTimer()
    // 	for i := 0; i < b.N; i++ {
    // 		d.population_variance()
    // 	}
    // }

    // // Test the incremental Variance function by itself. This result is how fast the
    // // Variance is calculated _including_ the time to incrementally update the Stats
    // // structure with 10 values. Therefore this result can be compared to the CalcVariance
    // // function operating on 10 values.
    // func BenchmarkPopulationVarWUpdates10(b *testing.B) {
    // 	b.StopTimer()
    //  let mut d = Stats::new();
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for _, v := range a {
    // 		d.Update(v)
    // 	}
    // 	b.StartTimer()
    // 	for i := 0; i < b.N; i++ {
    // 		d.population_variance()
    // 	}
    // }

    // // benchmark on 10 values, so divide by 10 to estimate the per-value time for array calculations
    // func BenchmarkCalcPopulationVariance10(b *testing.B) {
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for i := 0; i < b.N; i++ {
    // 		StatsPopulationVariance(a)
    // 	}
    // }

    // func BenchmarkCalcPopulationKurtosis10(b *testing.B) {
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for i := 0; i < b.N; i++ {
    // 		StatsPopulationKurtosis(a)
    // 	}
    // }

    // func BenchmarkCalcSampleKurtosis10(b *testing.B) {
    // 	a := []float64{1.0, -2.0, 13.0, 47.0, 115.0, -0.03, -123.4, 23.0, -23.04, 12.3}
    // 	for i := 0; i < b.N; i++ {
    // 		StatsSampleKurtosis(a)
    // 	}
    // }

    // // Find the time to calculate Sample Kurtosis on an input array 100k random values.
    // // The benchmark will repeat this test b.N times to determine a stable time. The
    // // resulting stable time is the time for the calculation on 100k values.
    // func BenchmarkCalcSampleKurtosis100k(b *testing.B) {
    // 	b.StopTimer()
    // 	rand.Seed(int64(time.Now().Nanosecond()))
    // 	n := 100000 // not the same as b.N
    // 	a := make([]float64, n)
    // 	for i := 0; i < n; i++ {
    // 		a[i] = rand.Float64()
    // 	}
    // 	b.StartTimer()
    // 	for i := 0; i < b.N; i++ {
    // 		StatsSampleKurtosis(a)
    // 	}
    // }

    //
    //
    // Degenerate examples tests
    //
    //

    #[test]
    // Update() 1 0 value
    fn test_update01() {
        let mut d = Stats::new();

        d.update(0.0);
        chk!(d.count(), 1u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance(), None);
        chk!(d.sample_variance(), None);
        chk!(d.population_standard_deviation(), None);
        chk!(d.sample_standard_deviation(), None);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 2 0 values
    fn test_update02() {
        let mut d = Stats::new();

        d.update(0.0);
        d.update(0.0);
        chk!(d.count(), 2u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance().unwrap(), 0.0);
        chk!(d.sample_variance().unwrap(), 0.0);
        chk!(d.population_standard_deviation().unwrap(), 0.0);
        chk!(d.sample_standard_deviation().unwrap(), 0.0);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 3 0 values.
    fn test_update03() {
        let mut d = Stats::new();

        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        chk!(d.count(), 3u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance().unwrap(), 0.0);
        chk!(d.sample_variance().unwrap(), 0.0);
        chk!(d.population_standard_deviation().unwrap(), 0.0);
        chk!(d.sample_standard_deviation().unwrap(), 0.0);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 4 0 values.
    fn test_update04() {
        let mut d = Stats::new();

        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        chk!(d.count(), 4u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance().unwrap(), 0.0);
        chk!(d.sample_variance().unwrap(), 0.0);
        chk!(d.population_standard_deviation().unwrap(), 0.0);
        chk!(d.sample_standard_deviation().unwrap(), 0.0);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 5 0 values.
    fn test_update05() {
        let mut d = Stats::new();

        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        d.update(0.0);
        chk!(d.count(), 5u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance().unwrap(), 0.0);
        chk!(d.sample_variance().unwrap(), 0.0);
        chk!(d.population_standard_deviation().unwrap(), 0.0);
        chk!(d.sample_standard_deviation().unwrap(), 0.0);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }

    #[test]
    // Update() 10 0 values.
    fn test_update010() {
        let mut d = Stats::new();

        let a = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for v in a {
            d.update(v)
        }
        chk!(d.count(), 10u32);
        chk!(d.min(), 0.0);
        chk!(d.max(), 0.0);
        chk!(d.sum(), 0.0);
        chk!(d.mean(), 0.0);
        chk!(d.population_variance().unwrap(), 0.0);
        chk!(d.sample_variance().unwrap(), 0.0);
        chk!(d.population_standard_deviation().unwrap(), 0.0);
        chk!(d.sample_standard_deviation().unwrap(), 0.0);
        chk!(d.population_skew(), None);
        chk!(d.sample_skew(), None);
        chk!(d.population_kurtosis(), None);
        chk!(d.sample_kurtosis(), None);
    }
}
