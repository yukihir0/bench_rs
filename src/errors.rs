use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum BenchmarkError {
    #[error("benchmark fail {cause:?}")]
    Fail { cause: String },
    #[error("benchmark penalty {cause:?} : {point:?}")]
    Penalty { cause: String, point: usize },
}

impl PartialEq for BenchmarkError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Clone)]
pub struct Errors {
    errors: Vec<BenchmarkError>,
}

impl Errors {
    pub fn new() -> Errors {
        Errors { errors: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BenchmarkError> {
        self.errors.iter()
    }

    pub fn record(&mut self, error: BenchmarkError) {
        self.errors.push(error);
    }

    pub fn total_penalty_point(&self) -> usize {
        self.errors.iter().fold(0, |total, error| match error {
            BenchmarkError::Penalty { cause: _, point } => total + point,
            _ => total,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::*;

    #[test]
    fn test_record() {
        let mut errors = Errors::new();

        let error1 = BenchmarkError::Fail {
            cause: "error1".into(),
        };
        let error2 = BenchmarkError::Fail {
            cause: "error2".into(),
        };
        let error3 = BenchmarkError::Fail {
            cause: "error3".into(),
        };
        let error4 = BenchmarkError::Penalty {
            cause: "error4".into(),
            point: 4,
        };
        let error5 = BenchmarkError::Penalty {
            cause: "error5".into(),
            point: 5,
        };
        let error6 = BenchmarkError::Penalty {
            cause: "error6".into(),
            point: 6,
        };

        errors.record(error1);
        errors.record(error2);
        errors.record(error3);
        errors.record(error4);
        errors.record(error5);
        errors.record(error6);

        assert_eq!(
            errors.errors[0].to_string(),
            BenchmarkError::Fail {
                cause: "error1".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors[1].to_string(),
            BenchmarkError::Fail {
                cause: "error2".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors[2].to_string(),
            BenchmarkError::Fail {
                cause: "error3".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors[3].to_string(),
            BenchmarkError::Penalty {
                cause: "error4".into(),
                point: 4
            }
            .to_string()
        );
        assert_eq!(
            errors.errors[4].to_string(),
            BenchmarkError::Penalty {
                cause: "error5".into(),
                point: 5
            }
            .to_string()
        );
        assert_eq!(
            errors.errors[5].to_string(),
            BenchmarkError::Penalty {
                cause: "error6".into(),
                point: 6
            }
            .to_string()
        );
    }

    #[test]
    fn test_total_penalty_point() {
        let mut errors = Errors::new();

        let error1 = BenchmarkError::Penalty {
            cause: "error1".into(),
            point: 1,
        };
        let error2 = BenchmarkError::Penalty {
            cause: "error2".into(),
            point: 2,
        };
        let error3 = BenchmarkError::Penalty {
            cause: "error3".into(),
            point: 3,
        };

        errors.record(error1);
        errors.record(error2);
        errors.record(error3);

        assert_eq!(errors.total_penalty_point(), 6);
    }
}
