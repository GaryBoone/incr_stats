use criterion::criterion_main;

mod all_stats_bench;
mod comparison_bench;

criterion_main! {
    all_stats_bench::benches,
    comparison_bench::benches,
}
