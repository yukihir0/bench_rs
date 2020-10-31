extern crate bench_rs;

use anyhow::Result;
use async_std;
use bench_rs::agent::*;
use bench_rs::benchmark::*;
use bench_rs::errors::*;
use bench_rs::score::*;

#[async_std::main]
async fn main() -> Result<()> {
    let agent = Agent::new("https://example.com");
    let score = Score::new();
    let errors = Errors::new();
    let benchmark = Benchmark::new(agent, score, errors);
    let benchmark_result = benchmark.start().await;

    println!("total: {}", benchmark_result.total_score());

    Ok(())
}
