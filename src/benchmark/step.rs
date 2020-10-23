use crate::score::*;

pub type BenchmarkStep = Box<dyn FnOnce(Score) -> BenchmarkStepResult>;

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

#[cfg(test)]
mod tests {
    use crate::benchmark::step::*;

    #[test]
    fn test_benchmark_step() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        let step: BenchmarkStep = Box::new(|score| -> BenchmarkStepResult {
            score.record("a");
            score.record("b");
            score.record("c");
            BenchmarkStepResult::new(score)
        });

        let benchmark_step_result = step(score);
        assert_eq!(benchmark_step_result.total_score(), 6);
    }
}
