use incr_stats::vec::Stats;

fn main() {
    let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

    // Initialize the structure with the data. It will be checked for invalid data such as NaNs or
    // Infs.
    let mut d = match Stats::new(&a) {
        Ok(d) => d,
        Err(e) => {
            println!("Data contains invalid values: {}", e);
            return;
        }
    };
    match d.mean() {
        Ok(v) => println!("The     mean is {:8.4}", v),
        Err(e) => println!("Error in mean: {}", e),
    }
    match d.sample_variance() {
        Ok(v) => println!("The variance is {:8.4}", v),
        Err(e) => println!("Error in variance: {}", e),
    }
    match d.sample_skewness() {
        Ok(v) => println!("The skewness is {:8.4}", v),
        Err(e) => println!("Error in skewness: {}", e),
    }
    match d.sample_kurtosis() {
        Ok(v) => println!("The kurtosis is {:8.4}", v),
        Err(e) => println!("Error in kurtosis: {}", e),
    }
}
