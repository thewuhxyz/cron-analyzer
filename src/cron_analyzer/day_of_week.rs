use super::{field::Field, error::CronAnalyzerError};

pub struct DayOfWeekField {
    pub raw: String,
}

impl<'a> Field<'a> for DayOfWeekField {
    fn raw(&self) -> String {
        self.raw.clone()
    }
    fn name(&self) -> String {
        "day-of-week".to_owned()
    }
    fn min(&self) -> usize {
        1
    }
    fn max(&self) -> usize {
        7
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        Some(vec![
            "",
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ])
    }

    fn convert_if_word(&self, input: &str) -> String {
        match input.to_lowercase().as_str() {
            "sun" | "sunday" => "1".to_owned(),
            "mon" | "monday" => "2".to_owned(),
            "tue" | "tues" | "tuesday" => "3".to_owned(),
            "wed" | "wednesday" => "4".to_owned(),
            "thu" | "thurs" | "thursday" => "5".to_owned(),
            "fri" | "friday" => "6".to_owned(),
            "sat" | "saturday" => "7".to_owned(),
            _ => input.to_owned(),
        }
    }

    fn analyze(&self) -> Result<String, CronAnalyzerError> {
        match self.raw.as_str() {
            "*" => Ok(format!("")),
            _ => match self.format_field(true) {
                Ok(s) => Ok(format!("on {s}")),
                Err(e) => Err(e),
            },
        }
    }
}