pub mod scenario;
pub mod step;

use crate::benchmark::scenario::*;
use crate::score::*;

pub struct Benchmark {
    score: Score,
    prepare_scenarios: Vec<BenchmarkScenario>,
    load_scenarios: Vec<BenchmarkScenario>,
    validation_scenarios: Vec<BenchmarkScenario>,
}

impl Benchmark {
    pub fn new(score: Score) -> Benchmark {
        Benchmark {
            score: score,
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

    pub fn start(self) -> BenchmarkResult {
        for scenario in self.prepare_scenarios {
            scenario.run(self.score.clone());
        }

        for scenario in self.load_scenarios {
            scenario.run(self.score.clone());
        }

        for scenario in self.validation_scenarios {
            scenario.run(self.score.clone());
        }

        BenchmarkResult::new(self.score)
    }
}

pub struct BenchmarkResult {
    score: Score,
}

impl BenchmarkResult {
    pub fn new(score: Score) -> BenchmarkResult {
        BenchmarkResult { score: score }
    }

    pub fn total_score(&self) -> usize {
        self.score.total()
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

    #[test]
    fn test_benchmark() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let mut benchmark = Benchmark::new(score.clone());

        let mut benchmark_scenario1 = BenchmarkScenario::new();
        benchmark_scenario1.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario1.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario1.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        }));

        let mut benchmark_scenario2 = BenchmarkScenario::new();
        benchmark_scenario2.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario2.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario2.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        }));

        let mut benchmark_scenario3 = BenchmarkScenario::new();
        benchmark_scenario3.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario3.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario3.add_benchmark_step(Box::new(|score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        }));

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start();
        assert_eq!(benchmark_result.total_score(), 18);
        assert_eq!(benchmark_result.is_success(), true);
        assert_eq!(benchmark_result.is_failure(), false);
    }
}
