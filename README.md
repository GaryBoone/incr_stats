
# incr_stats

## Incremental Descriptive Statistics for Rust 

`incr_stats` provides scalable descriptive statistics with the following features:
* Incremental (aks `streaming`) updates means that no data is stored
* Appropriate for low-memory applications or applications where the  dataset is too large to store
* Corresponding batch functions provided for each incremental statistic
* Both population and sample statistics
* Descriptive statistics: count, min, max, mean, variance, standard deviation, skewness, and kurtosis
* Protection against error-inducing NaNs and Infs
* Extensive tests for accuracy
* Validated against R and Octave
* Speed benchmarks compare incremental vs batch calculations

## Examples

### Incremental/Streaming
It's not necessary to store the entire data stream to calculate its descriptive statistics.

```rust
    let mut s = Stats::new();

    // Update the stats as data becomes available, without storing it.
    s.update(1.2)?;
    s.update(0.2)?;
    // ...

    // Calculate the descriptive statistics as needed.
    println!("The skewness is {:.4}", s.sample_skewness()?);
    println!("The kurtosis is {:.4}", s.sample_kurtosis()?);
    # Ok(0.0)
```
An `array_update()` function is also provided so that incremental updates can be performed on a group of values.

### Batch
The same function names can be used on stored data, providing the same accuracy.

```rust
    let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

    println!("The skewness is {:.4}", batch::sample_skewness(&a)?);
    println!("The kurtosis is {:.4}", batch::sample_kurtosis(&a)?);
    # Ok(0.0)
```

## Incremental vs Batch Updates
If minimal memory is required, use incremental statistics to calculate descriptive statistics without storing the entire dataset. Similarly, use the incremental statistics if the dataset is too large to store, such as in continuous data acquisition or streaming applications. 

If maximum speed is required for the statistical calculations, it's faster to operate on the stored data using the batch functions. That's because the incremental update function calculates several statistical moments for each datapoint.

## Population and Sample Statistics

## R and Octave Validation

## Speed

## Error Handling

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>