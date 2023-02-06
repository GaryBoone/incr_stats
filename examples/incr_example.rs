use rust_stats::incr::Stats;

fn main() {
    println!("Hello, world!");
    let mut s = Stats::new();
    s.update(1.0);
}
