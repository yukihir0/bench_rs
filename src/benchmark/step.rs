use crate::agent::*;
use crate::errors::*;
use crate::score::*;

use std::future::Future;
use std::pin::Pin;

pub type BoxFutBenchmarkStep = Pin<Box<dyn Future<Output = BenchmarkStepResult> + Send>>;
pub type BenchmarkStep = fn(Agent, Score, Errors) -> BoxFutBenchmarkStep;

#[derive(Clone)]
pub struct BenchmarkStepResult {
    score: Score,
    errors: Errors,
}

impl BenchmarkStepResult {
    pub fn new(score: Score, errors: Errors) -> BenchmarkStepResult {
        BenchmarkStepResult {
            score: score,
            errors: errors,
        }
    }

    pub fn total_score(&self) -> isize {
        self.total_gain() - self.total_lose()
    }

    pub fn total_gain(&self) -> isize {
        self.score.total() as isize
    }

    pub fn total_lose(&self) -> isize {
        self.errors.total_penalty_point() as isize
    }

    pub fn is_success(&self) -> bool {
        !self.is_failure()
    }

    pub fn is_failure(&self) -> bool {
        self.errors.iter().any(|error| match error {
            BenchmarkError::Fail { cause: _cause } => true,
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::step::*;
    use mockito;

    #[async_std::test]
    async fn test_benchmark_step() -> Result<(), ()> {
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

        fn step(agent: Agent, mut score: Score, mut errors: Errors) -> BoxFutBenchmarkStep {
            Box::pin(async move {
                let response = agent.get("/dummy").await;
                assert_eq!(response.unwrap().status(), surf::StatusCode::Ok);

                score.record("a");
                score.record("b");
                score.record("c");

                errors.record(BenchmarkError::Penalty {
                    cause: "error".into(),
                    point: 1,
                });

                BenchmarkStepResult::new(score, errors)
            })
        }

        let benchmark_step_result = step(agent, score, errors).await;
        assert_eq!(benchmark_step_result.total_score(), 5);
        assert_eq!(benchmark_step_result.total_gain(), 6);
        assert_eq!(benchmark_step_result.total_lose(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_step_result_is_success() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        fn step(agent: Agent, mut score: Score, mut errors: Errors) -> BoxFutBenchmarkStep {
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

        let benchmark_step_result = step(agent, score, errors).await;
        assert_eq!(benchmark_step_result.is_success(), true);
        assert_eq!(benchmark_step_result.is_failure(), false);

        Ok(())
    }

    #[async_std::test]
    async fn test_benchmark_step_result_is_failure() -> Result<(), ()> {
        let base_url = &mockito::server_url();
        let path = "/dummy";

        let _m = mockito::mock("GET", path)
            .with_status(surf::StatusCode::Ok as usize)
            .create();

        let agent = Agent::new(base_url);

        let mut score = Score::new();
        score.add_point_table("a", 1);

        let errors = Errors::new();

        fn step(agent: Agent, mut score: Score, mut errors: Errors) -> BoxFutBenchmarkStep {
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

        let benchmark_step_result = step(agent, score, errors).await;
        assert_eq!(benchmark_step_result.is_success(), false);
        assert_eq!(benchmark_step_result.is_failure(), true);

        Ok(())
    }
}
