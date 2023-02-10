use incr_stats::batch;

fn main() {
    let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

    // If the data could have invalid data such as NaNs or Infs, you can check it before calling the
    // other statistical calculations.
    match batch::validate(&a) {
        Ok(()) => {}
        Err(e) => {
            println!("Data contains invalid values: {}", e);
            return;
        }
    }
    match batch::mean(&a) {
        Ok(v) => println!("The     mean is {:.4}", v),
        Err(e) => println!("Error in mean: {}", e),
    }
    match batch::sample_variance(&a) {
        Ok(v) => println!("The variance is {:.4}", v),
        Err(e) => println!("Error in variance: {}", e),
    }
    match batch::sample_skewness(&a) {
        Ok(v) => println!("The skewness is {:.4}", v),
        Err(e) => println!("Error in skewness: {}", e),
    }
    match batch::sample_kurtosis(&a) {
        Ok(v) => println!("The kurtosis is {:.4}", v),
        Err(e) => println!("Error in kurtosis: {}", e),
    }
}
