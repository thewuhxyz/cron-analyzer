use super::{field::Field, error::CronAnalyzerError};

pub struct MinuteField {
    pub raw: String,
}

impl<'a> Field<'a> for MinuteField {
    fn raw(&self) -> String {
        self.raw.clone()
    }

    fn name(&self) -> String {
        "minute".to_owned()
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
        match self.raw.as_str() {
            "*" => Ok(format!("")),
            _ => {
              match self.format_field(false) {

                Ok(s) => Ok(format!("past {s}")),
                Err(e) => Err(e)
              }
            },
        }
    }
}