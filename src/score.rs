use std::collections::HashMap;

type PointName = String;
type PointUnit = usize;

#[derive(Clone)]
pub struct Score {
    point_table: HashMap<PointName, PointUnit>,
    records: Vec<PointName>,
}

impl Score {
    pub fn new() -> Score {
        Score {
            point_table: HashMap::new(),
            records: Vec::new(),
        }
    }

    pub fn add_point_table(&mut self, point_name: impl Into<PointName>, point_unit: PointUnit) {
        self.point_table.insert(point_name.into(), point_unit);
    }

    pub fn record(&mut self, point_name: impl Into<PointName>) {
        self.records.push(point_name.into());
    }

    pub fn total(&self) -> usize {
        self.records
            .iter()
            .fold(0, |total, record| total + self.point_table[record])
    }
}

#[cfg(test)]
mod tests {
    use crate::score::*;

    #[test]
    fn test_set() {
        let mut score = Score::new();
        score.add_point_table("a", 1);
        score.add_point_table("b", 2);
        score.add_point_table("c", 3);

        assert_eq!(score.point_table["a"], 1);
        assert_eq!(score.point_table["b"], 2);
        assert_eq!(score.point_table["c"], 3);
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
        score.add_point_table("a", 1);
        score.add_point_table("b", 2);
        score.add_point_table("c", 3);
        score.record("a");
        score.record("b");
        score.record("c");

        assert_eq!(score.total(), 6);
    }
}
