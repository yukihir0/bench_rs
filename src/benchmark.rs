pub mod scenario;
pub mod step;

use crate::agent::*;
use crate::benchmark::scenario::*;
use crate::errors::*;
use crate::score::*;

use async_std;
use async_std::task;
use crossbeam_channel::{bounded, unbounded, RecvError};

pub struct Benchmark {
    agent: Agent,
    score: Score,
    errors: Errors,
    prepare_scenarios: Vec<BenchmarkScenario>,
    load_scenarios: Vec<BenchmarkScenario>,
    validation_scenarios: Vec<BenchmarkScenario>,
    parallels: usize,
}

impl Benchmark {
    pub fn new(agent: Agent, score: Score, errors: Errors, parallels: usize) -> Benchmark {
        Benchmark {
            agent: agent,
            score: score,
            errors: errors,
            prepare_scenarios: Vec::new(),
            load_scenarios: Vec::new(),
            validation_scenarios: Vec::new(),
            parallels: parallels,
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

    async fn start_prepare_scenario(&self) -> Vec<BenchmarkScenarioResult> {
        let mut scenario_results = Vec::new();

        for scenario in self.prepare_scenarios.clone() {
            scenario_results.push(
                scenario
                    .run(self.agent.clone(), self.score.clone(), self.errors.clone())
                    .await,
            );
        }

        scenario_results
    }

    async fn start_load_scenario(&self) -> Vec<BenchmarkScenarioResult> {
        enum SourceMessage {
            Work(BenchmarkScenario),
            Stopped,
        }

        enum ProcessorMessage {
            Result(BenchmarkScenarioResult),
            Cancel(BenchmarkScenarioResult),
            Stopped,
        }

        let (work_sender, work_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();

        // Source
        let scenarios = self.load_scenarios.clone();
        let _source = task::spawn(async move {
            for scenario in scenarios {
                let scenario_name = scenario.clone().name;
                let _ = work_sender.send(SourceMessage::Work(scenario));
                log::debug!("[Source] send {}", scenario_name);
            }
            let _ = work_sender.send(SourceMessage::Stopped);
            log::debug!("[Source] send stop");
            drop(work_sender);
        });

        // Processor
        let parallels = self.parallels.clone();
        let agent = self.agent.clone();
        let score = self.score.clone();
        let errors = self.errors.clone();
        let _processor = task::spawn(async move {
            let (parallel_sender, parallel_receiver) = bounded(parallels);
            let (processor_result_sender, processor_result_receiver) = unbounded();
            let mut is_receive_exit = false;
            let mut ongoing_workers = 0;

            loop {
                crossbeam_channel::select! {
                    recv(work_receiver) -> scenario => {
                        match scenario {
                            Ok(SourceMessage::Work(scenario)) => {
                                let scenario_name = scenario.clone().name;
                                let result = BenchmarkScenarioResult::new(scenario_name);

                                if is_receive_exit {
                                    let _ = result_sender.send(ProcessorMessage::Cancel(result));
                                    continue;
                                }

                                let result_sender = result_sender.clone();
                                let processor_result_sender = processor_result_sender.clone();
                                let parallel_receiver = parallel_receiver.clone();

                                let _ = parallel_sender.send(());
                                ongoing_workers += 1;

                                let agent = agent.clone();
                                let score = score.clone();
                                let errors = errors.clone();
                                let _worker = task::spawn(async move {
                                    // task::sleep(std::time::Duration::from_secs(1)).await;
                                    let result = scenario
                                        .run(agent.clone(), score.clone(), errors.clone())
                                        .await;
                                    let _ = result_sender.send(ProcessorMessage::Result(result));
                                    let _ = processor_result_sender.send(());
                                    let _ = parallel_receiver.recv();
                                });
                            },
                            Ok(SourceMessage::Stopped) => {
                                is_receive_exit = true;
                                if ongoing_workers == 0 {
                                    let _ = result_sender.send(ProcessorMessage::Stopped);
                                    drop(result_sender);
                                    drop(processor_result_sender);
                                    drop(parallel_sender);
                                    break;
                                }
                            }
                            Err(RecvError) => {
                            },
                        }
                    },
                    recv(processor_result_receiver) -> msg => {
                        match msg {
                            Ok(()) => {
                                ongoing_workers -= 1;
                                if is_receive_exit && ongoing_workers == 0 {
                                    let _ = result_sender.send(ProcessorMessage::Stopped);
                                    drop(result_sender);
                                    drop(processor_result_sender);
                                    drop(parallel_sender);
                                    break;
                                }
                            },
                            Err(RecvError) => {
                            },
                        }
                    },
                }
            }
        });

        // Consumer
        let consumer = task::spawn(async move {
            let mut scenario_results = Vec::new();

            loop {
                match result_receiver.recv() {
                    Ok(ProcessorMessage::Result(result)) => {
                        let scenario_name = result.clone().scenario_name;
                        scenario_results.push(result);
                        log::debug!("[Consumer] receive result {}", scenario_name);
                    }
                    Ok(ProcessorMessage::Cancel(result)) => {
                        let scenario_name = result.clone().scenario_name;
                        log::debug!("[Consumer] receive cancel {}", scenario_name);
                    }
                    Ok(ProcessorMessage::Stopped) => {
                        log::debug!("[Consumer] receive stop");
                        break;
                    }
                    Err(RecvError) => {}
                }
            }

            scenario_results
        });

        consumer.await
    }

    async fn start_validation_scenario(&self) -> Vec<BenchmarkScenarioResult> {
        let mut scenario_results = Vec::new();

        for scenario in self.validation_scenarios.clone() {
            scenario_results.push(
                scenario
                    .run(self.agent.clone(), self.score.clone(), self.errors.clone())
                    .await,
            );
        }

        scenario_results
    }

    pub async fn start(&self) -> BenchmarkResult {
        let mut benchmark_result = BenchmarkResult::new();

        let _: Vec<_> = self
            .start_prepare_scenario()
            .await
            .into_iter()
            .map(|result| benchmark_result.add_scenario_result(result))
            .collect();

        let _: Vec<_> = self
            .start_load_scenario()
            .await
            .into_iter()
            .map(|result| benchmark_result.add_scenario_result(result))
            .collect();

        let _: Vec<_> = self
            .start_validation_scenario()
            .await
            .into_iter()
            .map(|result| benchmark_result.add_scenario_result(result))
            .collect();

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

    pub fn details(&self) -> Vec<BenchmarkScenarioResult> {
        self.scenario_results.clone()
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
        !self.is_failure()
    }

    pub fn is_failure(&self) -> bool {
        self.scenario_results
            .iter()
            .any(|result| result.is_failure())
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
        score.add_point_table("a", 1);
        score.add_point_table("b", 2);
        score.add_point_table("c", 3);

        let errors = Errors::new();

        let parallels = 8;

        let mut benchmark = Benchmark::new(agent, score, errors, parallels);

        fn step_a(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
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
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
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
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
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

        let mut benchmark_scenario1 = BenchmarkScenario::new("scenario1");
        benchmark_scenario1.add_benchmark_step(step_a);
        benchmark_scenario1.add_benchmark_step(step_b);
        benchmark_scenario1.add_benchmark_step(step_c);

        let mut benchmark_scenario2 = BenchmarkScenario::new("scenario2");
        benchmark_scenario2.add_benchmark_step(step_a);
        benchmark_scenario2.add_benchmark_step(step_b);
        benchmark_scenario2.add_benchmark_step(step_c);

        let mut benchmark_scenario3 = BenchmarkScenario::new("scenario3");
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

    #[async_std::test]
    async fn test_benchmark_result_is_success() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        let parallels = 8;

        let mut benchmark = Benchmark::new(agent, score, errors, parallels);

        fn step(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("a");

                errors.record(BenchmarkError::Penalty {
                    cause: "error".into(),
                    point: 1,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        let mut benchmark_scenario1 = BenchmarkScenario::new("scenario1");
        benchmark_scenario1.add_benchmark_step(step);

        let mut benchmark_scenario2 = BenchmarkScenario::new("scenario2");
        benchmark_scenario2.add_benchmark_step(step);

        let mut benchmark_scenario3 = BenchmarkScenario::new("scenario3");
        benchmark_scenario3.add_benchmark_step(step);

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start().await;
        assert_eq!(benchmark_result.is_success(), true);
        assert_eq!(benchmark_result.is_failure(), false);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_result_is_failure() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        let parallels = 8;

        let mut benchmark = Benchmark::new(agent, score, errors, parallels);

        fn step(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("a");

                errors.record(BenchmarkError::Fail {
                    cause: "error".into(),
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        let mut benchmark_scenario1 = BenchmarkScenario::new("scenario1");
        benchmark_scenario1.add_benchmark_step(step);

        let mut benchmark_scenario2 = BenchmarkScenario::new("scenario2");
        benchmark_scenario2.add_benchmark_step(step);

        let mut benchmark_scenario3 = BenchmarkScenario::new("scenario3");
        benchmark_scenario3.add_benchmark_step(step);

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start().await;
        assert_eq!(benchmark_result.is_success(), false);
        assert_eq!(benchmark_result.is_failure(), true);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_result_detail() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        let parallels = 8;

        let mut benchmark = Benchmark::new(agent, score, errors, parallels);

        fn step(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>> {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("a");

                errors.record(BenchmarkError::Penalty {
                    cause: "error".into(),
                    point: 1,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        let mut benchmark_scenario1 = BenchmarkScenario::new("scenario1");
        benchmark_scenario1.add_benchmark_step(step);

        let mut benchmark_scenario2 = BenchmarkScenario::new("scenario2");
        benchmark_scenario2.add_benchmark_step(step);

        let mut benchmark_scenario3 = BenchmarkScenario::new("scenario3");
        benchmark_scenario3.add_benchmark_step(step);

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start().await;
        assert_eq!(benchmark_result.details()[0].scenario_name(), "scenario1");
        assert_eq!(benchmark_result.details()[1].scenario_name(), "scenario2");
        assert_eq!(benchmark_result.details()[2].scenario_name(), "scenario3");

        Ok(())
    }
}
