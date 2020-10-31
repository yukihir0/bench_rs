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
        for step in self.steps {
            step(agent.clone(), score.clone(), errors.clone()).await;
        }

        BenchmarkScenarioResult::new(score, errors)
    }
}

pub struct BenchmarkScenarioResult {
    score: Score,
    errors: Errors,
}

impl BenchmarkScenarioResult {
    pub fn new(score: Score, errors: Errors) -> BenchmarkScenarioResult {
        BenchmarkScenarioResult {
            score: score,
            errors: errors,
        }
    }

    pub fn total_score(&self) -> usize {
        self.score.total() - self.errors.total_penalty_point()
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

        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let errors = Errors::new();

        let mut benchmark_scenario = BenchmarkScenario::new();

        fn step_a(
            agent: Agent,
            score: Score,
            errors: Errors,
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
            score: Score,
            errors: Errors,
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
            score: Score,
            errors: Errors,
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

        Ok(())
    }
}
