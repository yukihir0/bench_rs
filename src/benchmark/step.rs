use crate::agent::*;
use crate::errors::*;
use crate::score::*;

use std::future::Future;
use std::pin::Pin;

pub type BenchmarkStep =
    fn(Agent, Score, Errors) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>>;

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

    pub fn record(&self, key: impl Into<String>) {
        self.score.record(key);
    }

    pub fn total_score(&self) -> usize {
        self.score.total() - self.errors.total_penalty_point()
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

        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let errors = Errors::new();

        fn step(
            agent: Agent,
            score: Score,
            errors: Errors,
        ) -> Pin<Box<dyn Future<Output = BenchmarkStepResult>>> {
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

        Ok(())
    }
}
