
# incr_stats

## Fast Incremental Descriptive Statistics for Rust 

`incr_stats` provides time- and memory-optimized, scalable, descriptive statistics with the
following features:
* Incremental (aka `streaming`) updates mean that no source data is stored
* Faster than non-incremental equivalents, while requiring far less storage
* Allows fast update of already-calculated statistics
* Especially appropriate for low-memory applications or applications where the dataset is too large
  to store such as continuous data generation
* Descriptive statistics: count, min, max, mean, variance, standard deviation, skewness, and
  kurtosis
* Both population and sample statistics
* Protection against error-inducing NaNs and Infs
* Extensive tests for accuracy
* Validated against R and Octave
* Additional optimized batch functions provided for each statistic
* Equivalent textbook batch functions provided for each statistic
* Speed benchmarks compare incremental and batch calculations

## Examples

The `incr_stats` Stats functions operate on `f64` data and is easy to use:

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

The `incr_stats` crate contains two other versions of the same calculations for comparison and for
some scenarios where batch. In general, the incremental version is the fastest, but they all produce
identical results!

### Memoized
The `vec` version requires stored data, but is optimized and provides the same accuracy. Descriptive
statistics depend on each other, such as the skewness depending on the variance which depends on the
mean. This version ensures that only the minimal calculations are performed, no matter which
statistic is requested. Further, subsequent requests do not repeat already-done calculations.

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

Choose `incr` stats first, unless your use fits an optimization described below.

### `incr`, the incremental stats: 
The `incr` stats does not store the data, but instead stores only a few intermediate values needed
to calculate the complete statistics upon request. It is updated one or a few values at a time. 

In general, the incremental stats are equivalent in calculation to batch calculations, with the
calculations amortized over the individual data point updates. It's fastest way to calculate the descriptive statistics on large datasets.

It's also appropriate if:
* memory is limited
* the data is unlimited, such as in streaming or continuous data applications
* you don't want to store and manage all of the data
* the current stats need to be updated
* data becomes available only one or a few points at a time, but overall stats are needed

### `vec`, the optimized vector stats:
The tradeoff for eliminating data storage is that performing the intermediate calculations for each
datum is slower than simply storing the data.

The `vec` stats struct memoizes intermediate results while performing calculations on stored data.
This optimization ensures that dependent calculations such as the mean and variance are reused when
needed by other stats. Repeated calls become look-ups instead of recalculations.

Use the `vec` stats if:
* the application cannot provide enough time for the incremental `update()`s but only has time to
  store the data
* you're only interested in one or two statistics, so the overhead of the `update()` calculations is
  wasted on statistics that won't be requested

### `batch`, the unoptimized textbook equations:
The `batch` functions are mainly included for accuracy comparisons and testing, but they can be
slightly faster than the ``vec` versions which have some overhead due to checking for
previously-calculated values. If only one statistic is needed, then there's no reuse that would make
the `vec` version faster.

Use the `batch` stats if:
* the absolutely fastest calculation of a *single* statistic (eg, `sample_kurtosis`) is required

## Population and Sample Statistics
`incr_stats` provides both population and sample statistics. 

The code documents the corresponding functions in the [R](https://www.r-project.org) and [GNU
Octave](https://octave.org) stats packages, clarifying their naming and parameter differences. 

## Accuracy: R and Octave Validation

`incr_stats` unit tests include examples that are confirmed to match results of the
[R](https://www.r-project.org) and [GNU Octave](https://octave.org) stats packages to 13 decimal
places. 

See the code for the corresponding [R](https://www.r-project.org) and [GNU
Octave](https://octave.org) functions.

## Speed

The incremental statistics carry the overhead of doing the calculations of statistical moments for
each data point update. The overhead allows the complete descriptive statistics to be calculated
quickly at any time without storing the entire dataset. It may appear to be a considerable amount of
calculation for just one data point. And it may appear that a lot more calculation is done over all
of the points than is done in the batch cases. But in fact nearly identical calculations must be
done for the batch versions, so the two are nearly identical in terms of processing time. The batch
versions loop through the data several times; the incremental versions unroll these loops, splitting
each loop calculation into a separate `update()` performed when the point is acquired.

This is why the only reason to prefer batch statistics is because:
1. The data must be processed as quickly as possible at acquisition; there's only time to store it, or
1. You only need one or two of the statistics, so there's no need to calculate the intermediate values for the 4th moment at each `update()` when kurtosis will not be requested.

### Benchmarks

[Criterion](https://github.com/bheisler/criterion.rs) benchmarks are included to allow comparisons
of the incremental, memoized, and batch statistics calculations.

For datasets with 10 and 1,000,000 randomized values, here is the total time to calculate [lower is better]:

`Kurtosis` means that only the `sample_kurtosis` was calculated, showing the time if only one
statistic is needed. `All stats` means that all of the statistics (`min/max`, `mean`, `var`,
`{samp/pop}_skewness`, `{samp/pop}_kurtosis`, etc) were calculated.

| Method | 10, Kurtosis | 10, All stats | 1M, Kurtosis | 1M, All stats |
|--------|-------------:|--------------:|-------------:|--------------:|
| incr   |  81.736 ns   |   101.35 ns   |    7.004 ms  |     7.105 ms  |
| batch  |  40.124 ns   |   228.20 ns   |    3.830 ms  |    29.128 ms  |
| vec    |  68.094 ns   |   106.80 ns   |    4.124 ms  |     8.411 ms  |

### Benchmark Analysis

As expected, for calculating just one statistic, the `batch` code is fastest, independent of dataset
size. But if multiple stats are needed, the incremental version is fastest. As mentioned above, the
amount of calculation is similar in both the incremental and the optimized batch (`vec`) versions,
but the incremental version is faster than the optimized batch version by 5.1% (10 data points) to
15.5% (1M data points).

Note that for the incremental version on large (1M) datasets, the time required for calculating all
of the stats (7.105 ms) was very close to calculating just the sample kurtosis (7.004 ms) in this
benchmark run. This 1.4% difference is likely due to variation in the machine load and OS background
tasks during benchmarking. However, consider that for the incremental version, the majority of the
effort is spent on the individual point update calculations. As a result, the time required for
calculating the final `all stats` versus any individual statistic converges.

Comparing the mean to the all stats values 

## Error Handling

The `incr_stats` crate handles errors in a simple and consistent way. There are only three kinds of
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