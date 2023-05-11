use super::{error::CronAnalyzerError, field::Field};

pub struct SecondField {
    pub raw: String,
}

impl<'a> Field<'a> for SecondField {
    fn raw(&self) -> String {
        self.raw.clone()
    }

    fn name(&self) -> String {
        "second".to_owned()
    }
    fn min(&self) -> usize {
        0
    }
    fn max(&self) -> usize {
        59
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        None
    }

    fn convert_if_word(&self, input: &str) -> String {
        input.to_owned()
    }

    fn analyze(&self) -> Result<String, CronAnalyzerError> {
        match self.format_field(false) {
            Ok(s) => Ok(format!("{s}")),
            Err(e) => Err(e),
        }
    }
}
