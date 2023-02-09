use rust_stats::incr::Stats;

fn main() {
    println!("Hello, world!");
    let mut s = Stats::new();
    match s.update(1.0) {
        Ok(_) => println!("updated"),
        Err(e) => println!("Error: {}", e),
    }
}
