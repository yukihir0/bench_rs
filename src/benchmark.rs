pub mod scenario;
pub mod step;

use crate::agent::*;
use crate::benchmark::scenario::*;
use crate::errors::*;
use crate::score::*;

pub struct Benchmark {
    agent: Agent,
    score: Score,
    errors: Errors,
    prepare_scenarios: Vec<BenchmarkScenario>,
    load_scenarios: Vec<BenchmarkScenario>,
    validation_scenarios: Vec<BenchmarkScenario>,
}

impl Benchmark {
    pub fn new(agent: Agent, score: Score, errors: Errors) -> Benchmark {
        Benchmark {
            agent: agent,
            score: score,
            errors: errors,
            prepare_scenarios: Vec::new(),
            load_scenarios: Vec::new(),
            validation_scenarios: Vec::new(),
        }
    }

    pub fn add_prepare_scenario(&mut self, scenario: BenchmarkScenario) {
        self.prepare_scenarios.push(scenario);
    }

    pub fn add_load_scenario(&mut self, scenario: BenchmarkScenario) {
        self.load_scenarios.push(scenario);
    }

    pub fn add_validation_scenario(&mut self, scenario: BenchmarkScenario) {
        self.validation_scenarios.push(scenario);
    }

    pub async fn start(self) -> BenchmarkResult {
        let mut benchmark_result = BenchmarkResult::new();

        for scenario in self.prepare_scenarios {
            benchmark_result.add_scenario_result(
                scenario
                    .run(self.agent.clone(), self.score.clone(), self.errors.clone())
                    .await,
            );
        }

        for scenario in self.load_scenarios {
            benchmark_result.add_scenario_result(
                scenario
                    .run(self.agent.clone(), self.score.clone(), self.errors.clone())
                    .await,
            );
        }

        for scenario in self.validation_scenarios {
            benchmark_result.add_scenario_result(
                scenario
                    .run(self.agent.clone(), self.score.clone(), self.errors.clone())
                    .await,
            );
        }

        benchmark_result
    }
}

pub struct BenchmarkResult {
    scenario_results: Vec<BenchmarkScenarioResult>,
}

impl BenchmarkResult {
    pub fn new() -> BenchmarkResult {
        BenchmarkResult {
            scenario_results: Vec::new(),
        }
    }

    pub fn add_scenario_result(&mut self, result: BenchmarkScenarioResult) {
        self.scenario_results.push(result);
    }

    pub fn total_score(&self) -> isize {
        self.total_gain() - self.total_lose()
    }

    pub fn total_gain(&self) -> isize {
        self.scenario_results
            .iter()
            .fold(0, |total, result| total + result.total_gain())
    }

    pub fn total_lose(&self) -> isize {
        self.scenario_results
            .iter()
            .fold(0, |total, result| total + result.total_lose())
    }

    pub fn is_success(&self) -> bool {
        // TODO
        true
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::step::*;
    use crate::benchmark::*;
    use std::future::Future;
    use std::pin::Pin;

    #[async_std::test]
    async fn test_benchmark() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let errors = Errors::new();

        let mut benchmark = Benchmark::new(agent, score, errors);

        fn step_a(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("a");

                errors.record(BenchmarkError::Penalty {
                    cause: "error_a".into(),
                    point: 1,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        fn step_b(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("b");

                errors.record(BenchmarkError::Penalty {
                    cause: "error_b".into(),
                    point: 2,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        fn step_c(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("c");

                errors.record(BenchmarkError::Penalty {
                    cause: "error_c".into(),
                    point: 3,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        let mut benchmark_scenario1 = BenchmarkScenario::new();
        benchmark_scenario1.add_benchmark_step(step_a);
        benchmark_scenario1.add_benchmark_step(step_b);
        benchmark_scenario1.add_benchmark_step(step_c);

        let mut benchmark_scenario2 = BenchmarkScenario::new();
        benchmark_scenario2.add_benchmark_step(step_a);
        benchmark_scenario2.add_benchmark_step(step_b);
        benchmark_scenario2.add_benchmark_step(step_c);

        let mut benchmark_scenario3 = BenchmarkScenario::new();
        benchmark_scenario3.add_benchmark_step(step_a);
        benchmark_scenario3.add_benchmark_step(step_b);
        benchmark_scenario3.add_benchmark_step(step_c);

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start().await;
        assert_eq!(benchmark_result.total_score(), 0);
        assert_eq!(benchmark_result.total_gain(), 18);
        assert_eq!(benchmark_result.total_lose(), 18);
        assert_eq!(benchmark_result.is_success(), true);
        assert_eq!(benchmark_result.is_failure(), false);

        Ok(())
    }
}
