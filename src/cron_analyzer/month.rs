use super::{field::Field, error::CronAnalyzerError};

pub struct MonthField {
    pub raw: String,
}

impl<'a> Field<'a> for MonthField {
    fn raw(&self) -> String {
        self.raw.clone()
    }
    fn name(&self) -> String {
        "month".to_owned()
    }
    fn min(&self) -> usize {
        1
    }
    fn max(&self) -> usize {
        12
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        Some(vec![
            "",
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ])
    }

    fn convert_if_word(&self, input: &str) -> String {
        match input.to_lowercase().as_str() {
            "jan" | "january" => "1".to_owned(),
            "feb" | "february" => "2".to_owned(),
            "mar" | "march" => "3".to_owned(),
            "apr" | "april" => "4".to_owned(),
            "may" => "5".to_owned(),
            "jun" | "june" => "6".to_owned(),
            "jul" | "july" => "7".to_owned(),
            "aug" | "august" => "8".to_owned(),
            "sep" | "september" => "9".to_owned(),
            "oct" | "october" => "10".to_owned(),
            "nov" | "november" => "11".to_owned(),
            "dec" | "december" => "12".to_owned(),
            _ => input.to_owned(),
        }
    }

    fn analyze(&self) -> Result<String, CronAnalyzerError> {
        match self.raw.as_str() {
            "*" => Ok(format!("")),
            _ => match self.format_field(true) {
                Ok(s) => Ok(format!("in {s}")),
                Err(e) => Err(e),
            },
        }
    }
}