use crate::benchmark::step::*;
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

    pub fn run(self, score: Score) -> BenchmarkScenarioResult {
        for step in self.steps {
            step(score.clone());
        }

        BenchmarkScenarioResult::new(score)
    }
}

pub struct BenchmarkScenarioResult {
    score: Score,
}

impl BenchmarkScenarioResult {
    pub fn new(score: Score) -> BenchmarkScenarioResult {
        BenchmarkScenarioResult { score: score }
    }

    pub fn total_score(&self) -> usize {
        self.score.total()
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::scenario::*;

    #[test]
    fn test_benchmark_scenario() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let mut benchmark_scenario = BenchmarkScenario::new();
        benchmark_scenario.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        }));

        let benchmark_scenario_result = benchmark_scenario.run(score);
        assert_eq!(benchmark_scenario_result.total_score(), 6);
    }
}
