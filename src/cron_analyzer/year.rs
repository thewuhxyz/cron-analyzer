use super::{error::CronAnalyzerError, field::Field};

pub struct YearField {
    pub raw: String,
}

impl<'a> Field<'a> for YearField {
    fn raw(&self) -> String {
        self.raw.clone()
    }
    fn name(&self) -> String {
        "year".to_owned()
    }
    fn min(&self) -> usize {
        1970
    }
    fn max(&self) -> usize {
        2100
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        None
    }

    fn convert_if_word(&self, input: &str) -> String {
        input.to_owned()
    }

    fn analyze(&self) -> Result<String, CronAnalyzerError> {
        match self.raw.as_str() {
            "*" => Ok(format!("")),
            _ => match self.format_field(false) {
                Ok(s) => Ok(format!("in {s}")),
                Err(e) => Err(e),
            },
        }
    }
}
