extern crate bench_rs;

use async_std;
use bench_rs::benchmark::*;
use bench_rs::score::*;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let score = Score::new();
    let benchmark = Benchmark::new(score);
    let benchmark_result = benchmark.start();

    println!("total: {}", benchmark_result.total_score());

    Ok(())
}
