pub mod day_of_month;
pub mod day_of_week;
pub mod error;
pub mod field;
pub mod hour;
pub mod minute;
pub mod month;
pub mod second;
pub mod year;

pub use day_of_month::*;
pub use day_of_week::*;
pub use error::*;
pub use field::Field;
pub use hour::*;
pub use minute::*;
pub use month::*;
pub use second::*;
pub use year::*;

use regex::Regex;

pub struct CronAnalyzer {
    second: SecondField,
    minute: MinuteField,
    hour: HourField,
    day_of_month: DayOfMonthField,
    month: MonthField,
    day_of_week: DayOfWeekField,
    year: YearField,
}

impl CronAnalyzer {
    pub fn from_expr(expression: String) -> Result<String, CronAnalyzerError> {
        let split_expression = expression.trim().split_whitespace().collect::<Vec<&str>>();

        let no_of_fields = Self::in_range(split_expression.len())?;

        let second = SecondField {
            raw: split_expression[0].to_owned(),
        };
        let minute = MinuteField {
            raw: split_expression[1].to_owned(),
        };
        let hour = HourField {
            raw: split_expression[2].to_owned(),
        };
        let day_of_month = DayOfMonthField {
            raw: split_expression[3].to_owned(),
        };
        let month = MonthField {
            raw: split_expression[4].to_owned(),
        };
        let day_of_week = DayOfWeekField {
            raw: split_expression[5].to_owned(),
        };
        let year = YearField {
            raw: match no_of_fields {
                6 => "*".to_owned(),
                _ => split_expression[6].to_owned(),
            },
        };

        let analyzer = CronAnalyzer {
            second,
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
            year,
        };

        analyzer.analyze()
    }

    pub fn analyze(&self) -> Result<String, CronAnalyzerError> {
        let second = &self.second;
        let minute = &self.minute;
        let hour = &self.hour;
        let day_of_month = &self.day_of_month;
        let month = &self.month;
        let day_of_week = &self.day_of_week;
        let year = &self.year;

        let days_anded = hour.raw.starts_with("*") || month.raw.starts_with("*");

        let s = match !day_of_month.analyze()?.is_empty() && !day_of_week.analyze()?.is_empty() {
            false => "".to_owned(),
            true => match days_anded {
                false => "and".to_owned(),
                true => "if it's".to_owned(),
            },
        };

        let re = Regex::new(r"^0*\d\d?$").unwrap();

        let time: Option<[String; 3]> =
            match re.is_match(&second.raw) && re.is_match(&minute.raw) && re.is_match(&hour.raw) {
                true => {
                    let second = format!("0{}", &second.raw);
                    let minute = format!("0{}", &minute.raw);
                    let hour = format!("0{}", &hour.raw);
                    Some([
                        second[second.len() - 2..].to_owned(),
                        minute[minute.len() - 2..].to_owned(),
                        hour[hour.len() - 2..].to_owned(),
                    ])
                }
                false => None,
            };

        Ok(match time {
            Some(t) => {
                format!(
                    "At {}:{}:{} {} {} {} {} {}",
                    &t[2],
                    &t[1],
                    &t[0],
                    &day_of_month.analyze()?,
                    &s,
                    &day_of_week.analyze()?,
                    &month.analyze()?,
                    &year.analyze()?
                )
                .trim()
                .to_owned()
                    + "."
            }
            None => {
                format!(
                    "At {} {} {} {} {} {} {} {}",
                    &second.analyze()?,
                    &minute.analyze()?,
                    &hour.analyze()?,
                    &day_of_month.analyze()?,
                    &s,
                    &day_of_week.analyze()?,
                    &month.analyze()?,
                    &year.analyze()?
                )
                .trim()
                .to_owned()
                    + "."
            }
        })
    }

    fn in_range(check: usize) -> Result<usize, CronAnalyzerError> {
        match check >= 6 && check <= 7 {
            true => Ok(check),
            false => Err(CronAnalyzerError::new(format!(
                "Expression not within 6 - 7 fields"
            ))),
        }
    }
}
