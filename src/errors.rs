use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
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
pub struct BenchmarkErrors {
    errors: Arc<Mutex<Vec<BenchmarkError>>>,
}

impl BenchmarkErrors {
    pub fn new() -> BenchmarkErrors {
        BenchmarkErrors {
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_error(&self, error: BenchmarkError) {
        if let Ok(mut errors) = self.errors.lock() {
            errors.push(error);
        }
    }

    pub fn total(&self) -> usize {
        let mut total = 0;

        if let Ok(errors) = self.errors.lock() {
            for error in errors.iter() {
                match error {
                    BenchmarkError::Penalty { cause: _, point } => total += point,
                    _ => {}
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::*;
    use std::thread;

    #[test]
    fn test_add_error() {
        let errors = BenchmarkErrors::new();

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

        errors.add_error(error1);
        errors.add_error(error2);
        errors.add_error(error3);
        errors.add_error(error4);
        errors.add_error(error5);
        errors.add_error(error6);

        assert_eq!(
            errors.errors.lock().unwrap()[0].to_string(),
            BenchmarkError::Fail {
                cause: "error1".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors.lock().unwrap()[1].to_string(),
            BenchmarkError::Fail {
                cause: "error2".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors.lock().unwrap()[2].to_string(),
            BenchmarkError::Fail {
                cause: "error3".into()
            }
            .to_string()
        );
        assert_eq!(
            errors.errors.lock().unwrap()[3].to_string(),
            BenchmarkError::Penalty {
                cause: "error4".into(),
                point: 4
            }
            .to_string()
        );
        assert_eq!(
            errors.errors.lock().unwrap()[4].to_string(),
            BenchmarkError::Penalty {
                cause: "error5".into(),
                point: 5
            }
            .to_string()
        );
        assert_eq!(
            errors.errors.lock().unwrap()[5].to_string(),
            BenchmarkError::Penalty {
                cause: "error6".into(),
                point: 6
            }
            .to_string()
        );
    }

    #[test]
    fn test_total() {
        let errors = BenchmarkErrors::new();

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

        errors.add_error(error1);
        errors.add_error(error2);
        errors.add_error(error3);

        assert_eq!(errors.total(), 6);
    }

    #[test]
    fn test_add_error_with_thread() {
        let causes = vec![
            String::from("error1"),
            String::from("error2"),
            String::from("error3"),
        ];
        let errors = BenchmarkErrors::new();
        let mut handles = vec![];

        for cause in causes {
            let errors = errors.clone();
            let handle = thread::spawn(move || {
                errors.add_error(BenchmarkError::Fail { cause: cause });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            errors
                .errors
                .lock()
                .unwrap()
                .contains(&BenchmarkError::Fail {
                    cause: "error1".into()
                }),
            true
        );
        assert_eq!(
            errors
                .errors
                .lock()
                .unwrap()
                .contains(&BenchmarkError::Fail {
                    cause: "error2".into()
                }),
            true
        );
        assert_eq!(
            errors
                .errors
                .lock()
                .unwrap()
                .contains(&BenchmarkError::Fail {
                    cause: "error3".into()
                }),
            true
        );
    }

    #[test]
    fn test_total_with_thread() {
        let causes = vec![
            String::from("error1"),
            String::from("error2"),
            String::from("error3"),
        ];
        let errors = BenchmarkErrors::new();
        let mut handles = vec![];

        for (i, cause) in causes.into_iter().enumerate() {
            let errors = errors.clone();
            let handle = thread::spawn(move || {
                errors.add_error(BenchmarkError::Penalty {
                    cause: cause,
                    point: i + 1,
                });
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(errors.total(), 6);
    }
}
