use rust_stats::incr::Stats;

const DATA: [f64; 10] = [1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

fn main() {
    let mut s = Stats::new();

    // The data is validated on update, so NaNs or Infs are prevented from contaminating the data.
    match s.update(f64::NAN) {
        Ok(_) => println!("updated"),
        Err(e) => println!("Error when updating: {}", e),
    }

    // Suppose some process provides data. Instead of storing it, we can just characterize the data
    // as we observe it, greatly minimizing the memory required.
    for v in &DATA[..5] {
        match s.update(*v) {
            Ok(_) => {}
            Err(e) => println!("Error when updating: {}", e),
        }
    }

    // The stats can also be updated with an array of data if needed.
    match s.array_update(&DATA[5..]) {
        Ok(_) => {}
        Err(e) => println!("Error when updating: {}", e),
    }

    // After some updates, we can calculate the descriptive statistics.
    match s.mean() {
        Ok(v) => println!("The     mean is {:.4}", v),
        Err(e) => println!("Error in mean calculation: {}", e),
    }
    match s.sample_variance() {
        Ok(v) => println!("The variance is {:.4}", v),
        Err(e) => println!("Error in variance calculation: {}", e),
    }
    match s.sample_skewness() {
        Ok(v) => println!("The skewness is {:.4}", v),
        Err(e) => println!("Error in skewness calculation: {}", e),
    }
    match s.sample_kurtosis() {
        Ok(v) => println!("The kurtosis is {:.4}", v),
        Err(e) => println!("Error in kurtosis calculation: {}", e),
    }
}
