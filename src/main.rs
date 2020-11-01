extern crate bench_rs;
#[macro_use]
extern crate clap;
extern crate env_logger;

use anyhow::Result;
use async_std;
use bench_rs::agent::*;
use bench_rs::benchmark::scenario::*;
use bench_rs::benchmark::step::*;
use bench_rs::benchmark::*;
use bench_rs::errors::*;
use bench_rs::score::*;
use clap::{App, Arg};
use log;
use std::env;
use std::future::Future;
use std::pin::Pin;

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

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let agent = Agent::new(base_url);

    let mut score = Score::new();
    score.add_point_table("a", 1);

    let errors = Errors::new();

    fn prepare_step(
        _agent: Agent,
        score: Score,
        errors: Errors,
    ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
        Box::pin(async move { BenchmarkStepResult::new(score, errors) })
    }

    fn load_step(
        _agent: Agent,
        mut score: Score,
        mut errors: Errors,
    ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
        Box::pin(async move {
            score.record("a");

            errors.record(BenchmarkError::Penalty {
                cause: "error_a".into(),
                point: 1,
            });

            BenchmarkStepResult::new(score, errors)
        })
    }

    fn validation_step(
        _agent: Agent,
        score: Score,
        errors: Errors,
    ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
        Box::pin(async move { BenchmarkStepResult::new(score, errors) })
    }

    let mut prepare_scenario = BenchmarkScenario::new("prepare_scenario");
    prepare_scenario.add_benchmark_step(prepare_step);

    let mut load_scenario = BenchmarkScenario::new("load_scenario");
    load_scenario.add_benchmark_step(load_step);

    let mut validation_scenario = BenchmarkScenario::new("validation_scenario");
    validation_scenario.add_benchmark_step(validation_step);

    let mut benchmark = Benchmark::new(agent, score, errors);
    benchmark.add_prepare_scenario(prepare_scenario);
    benchmark.add_load_scenario(load_scenario);
    benchmark.add_validation_scenario(validation_scenario);

    let benchmark_result = benchmark.start().await;

    if benchmark_result.is_success() {
        log::info!(
            "Success / Score: {} ({} - {})",
            benchmark_result.total_score(),
            benchmark_result.total_gain(),
            benchmark_result.total_lose()
        );

        log::info!("Detail:");
        for result in benchmark_result.details() {
            log::info!(
                "  {} : {} ({} - {})",
                result.scenario_name(),
                result.total_score(),
                result.total_gain(),
                result.total_lose()
            );
        }
    } else {
        log::info!("Failure");
    }

    Ok(())
}
