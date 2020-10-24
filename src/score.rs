use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Score {
    criteria: Arc<Mutex<HashMap<String, usize>>>,
    records: Arc<Mutex<Vec<String>>>,
}

impl Score {
    pub fn new() -> Score {
        Score {
            criteria: Arc::new(Mutex::new(HashMap::new())),
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn set_criterion(&self, key: impl Into<String>, value: usize) {
        if let Ok(mut criteria) = self.criteria.lock() {
            criteria.insert(key.into(), value);
        }
    }

    pub fn record(&self, key: impl Into<String>) {
        if let Ok(mut records) = self.records.lock() {
            records.push(key.into());
        }
    }

    pub fn total(&self) -> usize {
        let mut total = 0;

        if let (Ok(criteria), Ok(records)) = (self.criteria.lock(), self.records.lock()) {
            for record in records.iter() {
                total += criteria[record];
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::score::*;
    use std::thread;

    #[test]
    fn test_set() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);

        assert_eq!(score.criteria.lock().unwrap()["a"], 1);
        assert_eq!(score.criteria.lock().unwrap()["b"], 2);
        assert_eq!(score.criteria.lock().unwrap()["c"], 3);
    }

    #[test]
    fn test_record() {
        let score = Score::new();
        score.record("a");
        score.record("b");
        score.record("c");

        assert_eq!(score.records.lock().unwrap()[0], "a");
        assert_eq!(score.records.lock().unwrap()[1], "b");
        assert_eq!(score.records.lock().unwrap()[2], "c");
    }

    #[test]
    fn test_total() {
        let score = Score::new();
        score.set_criterion("a", 1);
        score.set_criterion("b", 2);
        score.set_criterion("c", 3);
        score.record("a");
        score.record("b");
        score.record("c");

        assert_eq!(score.total(), 6);
    }

    #[test]
    fn test_set_with_thread() {
        let criteria = vec![String::from("a"), String::from("b"), String::from("c")];
        let score = Score::new();
        let mut handles = vec![];

        for (i, criterion) in criteria.iter().enumerate() {
            let score = score.clone();
            let criterion = criterion.clone();
            let handle = thread::spawn(move || {
                score.set_criterion(criterion, i + 1);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(score.criteria.lock().unwrap()["a"], 1);
        assert_eq!(score.criteria.lock().unwrap()["b"], 2);
        assert_eq!(score.criteria.lock().unwrap()["c"], 3);
    }

    #[test]
    fn test_record_with_thread() {
        let criteria = vec![String::from("a"), String::from("b"), String::from("c")];
        let score = Score::new();
        let mut handles = vec![];

        for criterion in criteria {
            let score = score.clone();
            let criterion = criterion.clone();
            let handle = thread::spawn(move || {
                score.record(criterion);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(score.records.lock().unwrap().len(), 3);
        assert_eq!(
            score.records.lock().unwrap().contains(&String::from("a")),
            true
        );
        assert_eq!(
            score.records.lock().unwrap().contains(&String::from("b")),
            true
        );
        assert_eq!(
            score.records.lock().unwrap().contains(&String::from("c")),
            true
        );
    }

    #[test]
    fn test_total_with_thread() {
        let criteria = vec![String::from("a"), String::from("b"), String::from("c")];
        let score = Score::new();
        let mut handles = vec![];

        for (i, criterion) in criteria.iter().enumerate() {
            let score = score.clone();
            let criterion = criterion.clone();
            let handle = thread::spawn(move || {
                score.set_criterion(criterion, i + 1);
            });
            handles.push(handle);
        }

        for criterion in criteria {
            let score = score.clone();
            let criterion = criterion.clone();
            let handle = thread::spawn(move || {
                score.record(criterion);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(score.total(), 6);
    }
}
