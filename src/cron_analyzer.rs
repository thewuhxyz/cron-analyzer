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
    pub fn from(expression: String) -> CronAnalyzer {
        let split_expression = expression.trim().split_whitespace().collect::<Vec<&str>>();

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
            raw: split_expression[6].to_owned(),
        };

        CronAnalyzer {
            second,
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
            year,
        }
    }

    pub fn analyze(&self) -> String {
        let second = &self.second;
        let minute = &self.minute;
        let hour = &self.hour;
        let day_of_month = &self.day_of_month;
        let month = &self.month;
        let day_of_week = &self.day_of_week;
        let year = &self.year;

        let days_anded = hour.raw.starts_with("*") || month.raw.starts_with("*");

        let s = match !day_of_month.analyze().is_empty() && !day_of_week.analyze().is_empty() {
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
                    let second = "0".to_owned() + &second.raw;
                    let minute = "0".to_owned() + &minute.raw;
                    let hour = "0".to_owned() + &hour.raw;
                    Some([
                        second[second.len() - 2..].to_owned(),
                        minute[minute.len() - 2..].to_owned(),
                        hour[hour.len() - 2..].to_owned(),
                    ])
                }
                false => None,
            };

        match time {
            Some(t) => {
                ("At ".to_owned()
                    + &t[2]
                    + ":"
                    + &t[1]
                    + ":"
                    + &t[0]
                    + " "
                    + &day_of_month.analyze()
                    + " "
                    + &s
                    + " "
                    + &day_of_week.analyze()
                    + " "
                    + &month.analyze()
                    + " "
                    + &year.analyze())
                    .trim()
                    .to_owned()
                    + "."
            }
            None => {
                ("At ".to_owned()
                    + &second.analyze()
                    + " "
                    + &minute.analyze()
                    + " "
                    + &hour.analyze()
                    + " "
                    + &day_of_month.analyze()
                    + " "
                    + &s
                    + " "
                    + &day_of_week.analyze()
                    + " "
                    + &month.analyze()
                    + " "
                    + &year.analyze())
                    .trim()
                    .to_owned()
                    + "."
            }
        }
    }
}

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

    fn analyze(&self) -> String {
        self.format_field(false)
    }
}

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

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => "past ".to_owned() + self.format_field(false).as_str(),
        }
    }
}

pub struct HourField {
    pub raw: String,
}

impl<'a> Field<'a> for HourField {
    fn raw(&self) -> String {
        self.raw.clone()
    }

    fn name(&self) -> String {
        "hour".to_owned()
    }
    fn min(&self) -> usize {
        0
    }
    fn max(&self) -> usize {
        23
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        None
    }

    fn convert_if_word(&self, input: &str) -> String {
        input.to_owned()
    }

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => "past ".to_owned() + self.format_field(false).as_str(),
        }
    }
}

pub struct DayOfMonthField {
    pub raw: String,
}

impl<'a> Field<'a> for DayOfMonthField {
    fn raw(&self) -> String {
        self.raw.clone()
    }

    fn name(&self) -> String {
        "day-of-month".to_owned()
    }
    fn min(&self) -> usize {
        1
    }
    fn max(&self) -> usize {
        31
    }
    fn selection(&self) -> Option<Vec<&'a str>> {
        None
    }

    fn convert_if_word(&self, input: &str) -> String {
        input.to_owned()
    }

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => " on ".to_owned() + self.format_field(false).as_str(),
        }
    }
}

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

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => " in ".to_owned() + self.format_field(true).as_str(),
        }
    }
}

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

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => " on ".to_owned() + self.format_field(true).as_str(),
        }
    }
}

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

    fn analyze(&self) -> String {
        match self.raw.as_str() {
            "*" => "".to_owned(),
            _ => " in ".to_owned() + self.format_field(false).as_str(),
        }
    }
}



pub trait Field<'a> {
    fn raw(&self) -> String;
    fn name(&self) -> String;
    fn min(&self) -> usize;
    fn max(&self) -> usize;
    fn selection(&self) -> Option<Vec<&'a str>>;
    fn convert_if_word(&self, input: &str) -> String;
    fn analyze(&self) -> String;

    fn suffix(&self, number: &str) -> String {
        match number.parse::<usize>() {
            Err(_) => String::from("Not a string"),
            Ok(num) => {
                // let some = if num > 20 { num % 10 } else { num };
                match num % 10 {
                    1 => num.to_string() + "st",
                    2 => num.to_string() + "nd",
                    3 => num.to_string() + "rd",
                    _ => num.to_string() + "th",
                }
            }
        }
    }

    fn format_field(&self, d_or_m: bool) -> String {
        let raw_string = self.raw();
        let name = self.name();
        let formatted_sections = raw_string
            .split(",")
            .map(|section| self.format_field_section(section))
            .collect::<Vec<String>>();
        let formatted_string = match formatted_sections.len() {
            0 => "".to_string(),
            1 => formatted_sections[0].to_string(),
            2 => (formatted_sections[0].to_owned() + "and" + &formatted_sections[1]).to_string(),
            _ => {
                formatted_sections[0..formatted_sections.len() - 1].join(",")
                    + ", and"
                    + &formatted_sections[formatted_sections.len() - 1]
            }
        };
        let some = match d_or_m {
            true => " ".to_owned(),
            false => name.to_owned() + " ",
        };
        println!("to cancel: {}{}", &some, &formatted_string);
        (some + &formatted_string)
            .replace("every 1st", "every")
            .replace((name.to_owned() + " every").as_str(), "every")
            .replace((", ".to_owned() + &name).as_str(), ", ")
            .replace((", and ".to_owned() + &name).as_str(), ", and ")
    }

    fn format_field_section(&self, section: &str) -> String {
        // let re = Regex::new(r"\d+|.").unwrap();
        // let some = re.find(field).expect("could not find match").as_str();

        let raw_string = section.to_owned();
        let selection = self.selection();
        let max = self.max();
        let name = self.name();

        if raw_string == "*" {
            return "every ".to_owned() + &self.name();
        } else {
            let re = Regex::new(r"\d+|\w+|.").unwrap();

            // let some = field.matches(r"\d+|.").collect::<Vec<&str>>();
            let sections = re
                .find_iter(&raw_string)
                .map(|m| self.convert_if_word(m.as_str()))
                .collect::<Vec<String>>();
            println!("some: {:?}", &sections);

            let date_from_selection = |index: usize| match &selection {
                None => index.to_string(),
                Some(v) => v[index].to_string(),
            };

            match sections[0].parse::<usize>() {
                Ok(index) => match sections.len() {
                    1 => "".to_owned() + &date_from_selection(index),
                    3 => {
                        // if let Ok(num) = some[2].parse::<usize>(){};
                        match sections[2].parse::<usize>() {
                            Err(_) => "".to_owned(),
                            Ok(num) => match sections[1].as_str() {
                                "/" => {
                                    "every ".to_owned()
                                        + &self.suffix(&sections[2])
                                        + " "
                                        + &name
                                        + " from "
                                        + &date_from_selection(index)
                                        + " through "
                                        + &date_from_selection(max)
                                }
                                "-" => {
                                    "every ".to_owned()
                                        + " "
                                        + &name
                                        + " from "
                                        + &date_from_selection(index)
                                        + " through "
                                        + &date_from_selection(num)
                                }
                                _ => "".to_owned(),
                            },
                        }
                    }
                    5 => {
                        let num = sections[2].parse::<usize>().unwrap();
                        match sections[1] == "-"
                            && num >= index
                            && sections[3] == "/"
                            && sections[4].parse::<usize>().unwrap() >= 1
                        {
                            true => {
                                "every ".to_owned()
                                    + &self.suffix(&sections[4])
                                    + " "
                                    + &name
                                    + " "
                                    + " from "
                                    + &date_from_selection(index)
                                    + " through "
                                    + &date_from_selection(num)
                            }
                            false => "".to_owned(),
                        }
                    }
                    _ => "".to_owned(),
                },
                Err(_) => {
                    match sections.len() == 3
                        && sections[1] == "/"
                        && sections[2].parse::<usize>().is_ok()
                        && sections[0] == "*"
                    {
                        true => {
                            "every ".to_owned() + " " + &self.suffix(&sections[2]) + " " + &name
                        }
                        false => "".to_owned(),
                    }
                }
            }
        }
    }
}
