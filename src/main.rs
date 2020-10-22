extern crate bench_rs;

use bench_rs::benchmark::*;
use bench_rs::score::*;

fn main() {
    let score = Score::new();
    let benchmark = Benchmark::new(score);
    let benchmark_result = benchmark.start();

    println!("total: {}", benchmark_result.total_score());
}
