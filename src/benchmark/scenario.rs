use crate::agent::*;
use crate::benchmark::step::*;
use crate::errors::*;
use crate::score::*;

pub struct BenchmarkScenario {
    steps: Vec<BenchmarkStep>,
}

impl BenchmarkScenario {
    pub fn new() -> BenchmarkScenario {
        BenchmarkScenario { steps: Vec::new() }
    }

    pub fn add_benchmark_step(&mut self, step: BenchmarkStep) {
        self.steps.push(step);
    }

    pub async fn run(self, agent: Agent, score: Score, errors: Errors) -> BenchmarkScenarioResult {
        let mut scenario_result = BenchmarkScenarioResult::new();

        for step in self.steps {
            scenario_result
                .add_step_result(step(agent.clone(), score.clone(), errors.clone()).await);
        }

        scenario_result
    }
}

pub struct BenchmarkScenarioResult {
    step_results: Vec<BenchmarkStepResult>,
}

impl BenchmarkScenarioResult {
    pub fn new() -> BenchmarkScenarioResult {
        BenchmarkScenarioResult {
            step_results: Vec::new(),
        }
    }

    pub fn add_step_result(&mut self, result: BenchmarkStepResult) {
        self.step_results.push(result);
    }

    pub fn total_score(&self) -> isize {
        self.total_gain() - self.total_lose()
    }

    pub fn total_gain(&self) -> isize {
        self.step_results
            .iter()
            .fold(0, |total, result| total + result.total_gain())
    }

    pub fn total_lose(&self) -> isize {
        self.step_results
            .iter()
            .fold(0, |total, result| total + result.total_lose())
    }

    pub fn is_success(&self) -> bool {
        !self.is_failure()
    }

    pub fn is_failure(&self) -> bool {
        self.step_results.iter().any(|result| result.is_failure())
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::scenario::*;
    use mockito;
    use std::future::Future;
    use std::pin::Pin;

    #[async_std::test]
    async fn test_benchmark_scenario() -> Result<(), ()> {
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

        let mut benchmark_scenario = BenchmarkScenario::new();

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

        benchmark_scenario.add_benchmark_step(step_a);
        benchmark_scenario.add_benchmark_step(step_b);
        benchmark_scenario.add_benchmark_step(step_c);

        let benchmark_scenario_result = benchmark_scenario.run(agent, score, errors).await;
        assert_eq!(benchmark_scenario_result.total_score(), 0);
        assert_eq!(benchmark_scenario_result.total_gain(), 6);
        assert_eq!(benchmark_scenario_result.total_lose(), 6);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_scenario_result_is_success() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        let mut benchmark_scenario = BenchmarkScenario::new();

        fn step(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
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

        benchmark_scenario.add_benchmark_step(step);

        let benchmark_scenario_result = benchmark_scenario.run(agent, score, errors).await;
        assert_eq!(benchmark_scenario_result.is_success(), true);
        assert_eq!(benchmark_scenario_result.is_failure(), false);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_scenario_result_is_failure() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        let mut benchmark_scenario = BenchmarkScenario::new();

        fn step(
            agent: Agent,
            mut score: Score,
            mut errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
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

        benchmark_scenario.add_benchmark_step(step);

        let benchmark_scenario_result = benchmark_scenario.run(agent, score, errors).await;
        assert_eq!(benchmark_scenario_result.is_success(), false);
        assert_eq!(benchmark_scenario_result.is_failure(), true);

        Ok(())
    }
}
