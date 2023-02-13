
# fast_stats

## Fast Incremental Descriptive Statistics for Rust 

`fast_stats` provides time- and memory-optimized, scalable, descriptive statistics with the
following features:
* Incremental (aka `streaming`) updates mean that no source data is stored, appropriate for low-memory applications or applications where the  dataset is too large to store
* Fully memoized calculations mean no calculations are repeated for fast results
* Equivalent textbook batch functions provided for each statistic
* Both population and sample statistics
* Descriptive statistics: count, min, max, mean, variance, standard deviation, skewness, and
  kurtosis
* Protection against error-inducing NaNs and Infs
* Extensive tests for accuracy
* Validated against R and Octave
* Speed benchmarks compare incremental, memoized, and batch calculations

## Examples
The `fast_stats` crate contains three versions of the same calculations. Choose the one that best
provides the optimization you need. Details below. If you're not sure, use any one: they all produce
identical results!

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

### Memoized
Equivalent functions are also available for stored data, providing the same accuracy, but optimized
for speed over batch data. Descriptive statistics depend on each other, such as the skewness
depending on the variance which depends on the mean. This version ensures that only the minimal
calculations are performed, no matter which statistic is requested. Further, subsequent requests do
not repeat already-done calculations.

```rust
let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];
let mut s = Desc::new(&a)?;

println!("The skewness is {:.4}", d.sample_skewness()?);
println!("The kurtosis is {:.4}", d.sample_kurtosis()?);
```
### Batch
Finally, a third set of functions use traditional, textbook calculations for comparison. These do
the required calculations with no other overhead. They are included primarily for comparison and
testing, but can be optimal if only one or two statistics is needed.

```rust
let a = vec![1.2, -1.0, 2.3, 10.0, -3.0, 3.2, 0.33, 0.23, 0.23, 1.0];

println!("The skewness is {:.4}", batch::sample_skewness(&a)?);
println!("The kurtosis is {:.4}", batch::sample_kurtosis(&a)?);
```

## Which to use?
The `incr` stats does not store the data, but only a few intermediate values needed to calculate the
stats. It is updated one or a few values at a time. The tradeoff for eliminating data storage is
that performing the intermediate calculations for each datum is slower than simply storing the data.

Use `incr` stats if:
* memory is limited
* the data is unlimited, such as in streaming or continuous data applications
* you don't want to store and manage all of the data
* the current stats need to be updated
* data becomes available only one or a few points at a time, but overall stats are needed

The `desc` stats struct memoizes intermediate results while performing calculations on stored data.
This optimization ensures that dependent calculations such as the mean and variance are reused when
needed by other stats. Repeated calls become look-ups instead of recalculations.

Use the `desc` stats if:
* all of the data for analysis is present in memory
* maximum speed is required for the statistical calculations of stored data and multiple statistics
  are needed
* 
* large enough batches of data are acquired that the variation between samples is small enough to
  ignore
* there's not enough time for the incremental `update()`s and only time to store the data

The `batch` functions are mainly included for accuracy comparisons and testing, but they can be
faster than the ``desc` versions which have some overhead due to checking for previously-calculated
values. If only one statistic is needed, then there's no reuse that would make the `desc` version
faster.

Use the `batch` stats if:
* there's sufficient memory to store data
* the absolutely fastest calculation of a *single* statistic is required

## Population and Sample Statistics
`fast_stats` provides both population and sample statistics. 

The code documents the corresponding functions in the [R](https://www.r-project.org) and [GNU
Octave](https://octave.org) stats packages, clarifying their naming and parameter differences. 

## R and Octave Validation

`fast_stats` unit tests include examples that are confirmed to match results of the
[R](https://www.r-project.org) and [GNU Octave](https://octave.org) stats packages to 13 decimal
places. See the code for the corresponding [R](https://www.r-project.org) and [GNU
Octave](https://octave.org) functions.

## Speed

[Criterion](https://github.com/bheisler/criterion.rs) benchmarks are included to allow comparisons
of the incremental, memoized, and batch statistics calculations.

The incremental statistic carry the overhead of doing the calculations of statistical moments for
each data point update. The result is that the complete descriptive statistics can be calculated at
any time. However, this overhead adds up, especially if the complete calculations are not often
requested. 

The `desc` versions require stored data, but are optimized for rapid calculation of multiple
statistics. 

The batch calculations are faster for larger amounts of data, if storage is available, for any
single statistic, such as mean or skewness. But if you need more than one statistic, use the `desc`
stats to take advantage of the reuse of the other statistics' calculations.

This crate is designed to efficiently calculate incremental statistics. Although batch versions of
the statistics are provided, they aren't optimized for 

## Error Handling

The `fast_stats` crate handles errors in a simple and consistent way. There are only three kinds of
errors:
1. `NotEnoughData`: This error merely means that more data is needed to allow the calculation of the
   statistic. For example, the sample skewness calculation includes a division by `n-1` so must
   include at least 2 data points to avoid a division by 0.0. 
1. `Undefined`: Even with enough valid data, some statistics produce undefined results. For example,
   if all of the data is the same value, the variance is 0.0. Skewness and kurtosis, which are
   measures of the distribution around a central tendency, don't exist. This fact is reflected in
   the calculations by a division by the variance (ie a divide by 0.0). These are therefore
   undefined.
1. `InvalidData`: The floating data is checked for NaNs and Infs from the `IEEE 754` standard.

Callers that don't need to make these distinctions can just react to any error.

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