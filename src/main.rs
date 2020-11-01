extern crate bench_rs;

#[macro_use]
extern crate clap;

use anyhow::Result;
use async_std;
use bench_rs::agent::*;
use bench_rs::benchmark::*;
use bench_rs::errors::*;
use bench_rs::score::*;
use clap::{App, Arg};

#[async_std::main]
async fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("base_url")
                .about("benchmark target base url")
                .short('b')
                .long("base_url")
                .value_name("BASE_URL")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let base_url = matches.value_of("base_url").unwrap();

    let agent = Agent::new(base_url);
    let score = Score::new();
    let errors = Errors::new();
    let benchmark = Benchmark::new(agent, score, errors);
    let benchmark_result = benchmark.start().await;

    println!("total: {}", benchmark_result.total_score());
    println!("gain: {}", benchmark_result.total_gain());
    println!("lose: {}", benchmark_result.total_lose());

    Ok(())
}
