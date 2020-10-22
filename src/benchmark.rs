use crate::score::Score;

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

type BenchmarkStep = Box<dyn FnOnce(Score) -> BenchmarkStepResult>;

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

pub struct BenchmarkStepResult {
    score: Score,
}

impl BenchmarkStepResult {
    pub fn new(score: Score) -> BenchmarkStepResult {
        BenchmarkStepResult { score: score }
    }

    pub fn record<S>(&self, key: S)
    where
        S: Into<String>,
    {
        self.score.record(key);
    }

    pub fn total_score(&self) -> usize {
        self.score.total()
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
    use crate::benchmark::*;

    #[test]
    fn test_benchmark_scenario() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let mut benchmark_scenario = BenchmarkScenario::new();
        benchmark_scenario.add_benchmark_step(Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario.add_benchmark_step(Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        }));
        benchmark_scenario.add_benchmark_step(Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        }));

        let benchmark_scenario_result = benchmark_scenario.run(score);
        assert_eq!(benchmark_scenario_result.total_score(), 6);
    }

    #[test]
    fn test_benchmark() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let mut benchmark = Benchmark::new(score.clone());

        let benchmark_step1 = Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("a");
            BenchmarkStepResult::new(score)
        });
        let benchmark_step2 = Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("b");
            BenchmarkStepResult::new(score)
        });
        let benchmark_step3 = Box::new(|score: Score| -> BenchmarkStepResult {
            score.record("c");
            BenchmarkStepResult::new(score)
        });

        let mut benchmark_scenario1 = BenchmarkScenario::new();
        benchmark_scenario1.add_benchmark_step(benchmark_step1.clone());
        benchmark_scenario1.add_benchmark_step(benchmark_step2.clone());
        benchmark_scenario1.add_benchmark_step(benchmark_step3.clone());

        let mut benchmark_scenario2 = BenchmarkScenario::new();
        benchmark_scenario2.add_benchmark_step(benchmark_step1.clone());
        benchmark_scenario2.add_benchmark_step(benchmark_step2.clone());
        benchmark_scenario2.add_benchmark_step(benchmark_step3.clone());

        let mut benchmark_scenario3 = BenchmarkScenario::new();
        benchmark_scenario3.add_benchmark_step(benchmark_step1.clone());
        benchmark_scenario3.add_benchmark_step(benchmark_step2.clone());
        benchmark_scenario3.add_benchmark_step(benchmark_step3.clone());

        benchmark.add_prepare_scenario(benchmark_scenario1);
        benchmark.add_load_scenario(benchmark_scenario2);
        benchmark.add_validation_scenario(benchmark_scenario3);

        let benchmark_result = benchmark.start();
        assert_eq!(benchmark_result.total_score(), 18);
        assert_eq!(benchmark_result.is_success(), true);
        assert_eq!(benchmark_result.is_failure(), false);
    }
}
