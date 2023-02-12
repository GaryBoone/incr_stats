
# incr_stats

## Incremental Descriptive Statistics for Rust 

`incr_stats` provides scalable descriptive statistics with the following features:
* Incremental (aka `streaming`) updates mean that no data is stored
* Appropriate for low-memory applications or applications where the  dataset is too large to store
* Equivalent batch functions provided for each incremental statistic
* Both population and sample statistics
* Descriptive statistics: count, min, max, mean, variance, standard deviation, skewness, and
  kurtosis
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
```
An `array_update()` function is also provided so that incremental updates can be performed on a
group of values.

### Batch
The same function names are also available for stored data, providing the same accuracy.

```rust
let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

println!("The skewness is {:.4}", batch::sample_skewness(&a)?);
println!("The kurtosis is {:.4}", batch::sample_kurtosis(&a)?);
```

## Incremental vs Batch Updates
If minimal memory is required, use incremental statistics to calculate descriptive statistics
without storing the entire dataset. Similarly, use the incremental statistics if the dataset is too
large to store, such as in continuous data acquisition or streaming applications. 

If maximum speed is required for the statistical calculations, it's faster to operate on the stored
data using the batch functions. That's because the incremental update function calculates several
statistical moments for each datapoint.

## Population and Sample Statistics
`incr_stats` provides both population and sample statistics. 

The code documents the corresponding functions in the [R](https://www.r-project.org) and [GNU
Octave](https://octave.org) stats packages, clarifying their naming and parameter differences. 

## R and Octave Validation

`incr_stats` unit tests include examples that are confirmed to match results of the
[R](https://www.r-project.org) and [GNU Octave](https://octave.org) stats packages to 13 decimal
places.

## Speed

[Criterion](https://github.com/bheisler/criterion.rs) benchmarks are included to allow comparisons
of the incremental and batch statistics calculations.

The incremental statistic carry the overhead of doing the calculations of statistical moments for
each data point update. The result is that the complete descriptive statistics can be calculated at
any time. However, this overhead adds up, especially if the complete calculations are not often
requested. The batch calculations are faster for larger amounts of data, if storage is available,
for any single statistic, such as mean or skewness.

For multiple statistics, a `descriptive_statistics` function is provided. It efficiently calculates
all of the basic statistics.  

This crate is designed to efficiently calculate incremental statistics. Although batch versions of the
statistics are provided, they aren't optimized for 

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