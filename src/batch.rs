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
        if v > max {
            max = *v;
        }
    }
    Some(max)
}

// fn stats_sum(data: &[f64]) (sum float64) {
// 	for v in data {
// 		sum += v
// 	}
// 	return
// }

// fn stats_mean(data: &[f64]) -> f64 {
// 	return StatsSum(data) / float64(len(data))
// }

// func sumSquaredDeltas(data []float64) (ssd float64) {
// 	mean := StatsMean(data)
// 	for v in data {
// 		delta := v - mean
// 		ssd += delta * delta
// 	}
// 	return
// }

// fn stats_population_variance(data: &[f64]) -> f64 {
// 	n := float64(len(data))
// 	ssd := sumSquaredDeltas(data)
// 	return ssd / n
// }

// fn stats_sample_variance(data: &[f64]) -> f64 {
// 	n := float64(len(data))
// 	ssd := sumSquaredDeltas(data)
// 	return ssd / (n - 1.0)
// }

// fn stats_population_standard_deviation(data: &[f64]) -> f64 {
// 	return math.Sqrt(StatsPopulationVariance(data))
// }

// fn stats_sample_standard_deviation(data: &[f64]) -> f64 {
// 	return math.Sqrt(StatsSampleVariance(data))
// }

// fn stats_population_skew(data: &[f64]) (skew float64) {
// 	mean := StatsMean(data)
// 	n := float64(len(data))

// 	sum3 := 0.0
// 	for v in data {
// 		delta := v - mean
// 		sum3 += delta * delta * delta
// 	}

// 	variance := math.Sqrt(StatsPopulationVariance(data))
// 	skew = sum3 / n / (variance * variance * variance)
// 	return
// }

// fn stats_sample_skew(data: &[f64]) -> f64 {
// 	popSkew := StatsPopulationSkew(data)
// 	n := float64(len(data))
// 	return math.Sqrt(n*(n-1.0)) / (n - 2.0) * popSkew
// }

// // The kurtosis functions return _excess_ kurtosis
// fn stats_population_kurtosis(data: &[f64]) (kurtosis float64) {
// 	mean := StatsMean(data)
// 	n := float64(len(data))

// 	sum4 := 0.0
// 	for v in data {
// 		delta := v - mean
// 		sum4 += delta * delta * delta * delta
// 	}

// 	variance := StatsPopulationVariance(data)
// 	kurtosis = sum4/(variance*variance)/n - 3.0
// 	return
// }

// fn stats_sample_kurtosis(data: &[f64]) -> f64 {
// 	populationKurtosis := StatsPopulationKurtosis(data)
// 	n := float64(len(data))
// 	return (n - 1.0) / ((n - 2.0) * (n - 3.0)) * ((n+1.0)*populationKurtosis + 6.0)
// }
