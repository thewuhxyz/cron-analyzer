use std::{error, fmt };

#[derive(Debug)]
pub struct CronAnalyzerError {
  msg: String,
}

impl CronAnalyzerError {
  pub fn new(msg: String) -> Self {
    CronAnalyzerError { msg }
  }
}

impl error::Error for CronAnalyzerError {}

impl fmt::Display for CronAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

