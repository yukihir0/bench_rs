use std::collections::HashMap;

#[derive(Clone)]
pub struct Score {
    criteria: HashMap<String, usize>,
    records: Vec<String>,
}

impl Score {
    pub fn new() -> Score {
        Score {
            criteria: HashMap::new(),
            records: Vec::new(),
        }
    }

    pub fn set_criterion(&mut self, key: impl Into<String>, value: usize) {
        self.criteria.insert(key.into(), value);
    }

    pub fn record(&mut self, key: impl Into<String>) {
        self.records.push(key.into());
    }

    pub fn total(&self) -> usize {
        self.records
            .iter()
            .fold(0, |total, record| total + self.criteria[record])
    }
}

#[cfg(test)]
mod tests {
    use crate::score::*;

    #[test]
    fn test_set() {
        let mut score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        assert_eq!(score.criteria["a"], 1);
        assert_eq!(score.criteria["b"], 2);
        assert_eq!(score.criteria["c"], 3);
    }

    #[test]
    fn test_record() {
        let mut score = Score::new();
        score.record("a");
        score.record("b");
        score.record("c");

        assert_eq!(score.records[0], "a");
        assert_eq!(score.records[1], "b");
        assert_eq!(score.records[2], "c");
    }

    #[test]
    fn test_total() {
        let mut score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);
        score.record("a");
        score.record("b");
        score.record("c");

        assert_eq!(score.total(), 6);
    }
}
