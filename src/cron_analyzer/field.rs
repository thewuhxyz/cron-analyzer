use regex::Regex;

use super::error::CronAnalyzerError;

pub trait Field<'a> {
    fn raw(&self) -> String;
    fn name(&self) -> String;
    fn min(&self) -> usize;
    fn max(&self) -> usize;
    fn selection(&self) -> Option<Vec<&'a str>>;
    fn convert_if_word(&self, input: &str) -> String;
    fn analyze(&self) -> Result<String, CronAnalyzerError>;

    fn in_range(&self, check: usize) -> Result<(), CronAnalyzerError> {
        match self.min() <= check && check <= self.max() {
            false => Err(CronAnalyzerError::new(format!(
                "Input '{check}' not within '{}' range",
                self.name()
            ))),
            true => Ok(()),
        }
    }

    fn suffix(&self, number: &str) -> Result<String, CronAnalyzerError> {
        match number.parse::<usize>() {
            Err(_) => Err(CronAnalyzerError::new(format!(
                "'{number}' not a number at {}",
                &self.name()
            ))),
            Ok(num) => Ok(match num % 10 {
                1 => num.to_string() + "st",
                2 => num.to_string() + "nd",
                3 => num.to_string() + "rd",
                _ => num.to_string() + "th",
            }),
        }
    }

    fn format_field(&self, day_of: bool) -> Result<String, CronAnalyzerError> {
        let raw_string = self.raw();
        let name = self.name();
        let sections: Result<Vec<String>, CronAnalyzerError> = raw_string
            .split(",")
            .map(|section| self.format_field_section(section))
            .collect();

        match sections {
            Ok(formatted_sections) => {
                let formatted_string = match formatted_sections.len() {
                    0 => format!(""),
                    1 => format!("{}", formatted_sections[0].to_string()),
                    2 => format!("{} and {}", &formatted_sections[0], &formatted_sections[1]),
                    _ => {
                        format!(
                            "{}, and {}",
                            &formatted_sections[0..formatted_sections.len() - 1].join(", "),
                            &formatted_sections[formatted_sections.len() - 1]
                        )
                    }
                };
                let s = match day_of {
                    true => "".to_owned(),
                    false => format!("{} ", &name),
                };

                Ok(format!("{}{}", &s, &formatted_string)
                    .replace("every 1st", "every")
                    .replace(&format!("{} every", &name), "every")
                    .replace(&format!(", {}", &name), ",")
                    .replace(&format!(", and {}", &name), ", and"))
            }
            Err(e) => Err(e),
        }
    }

    fn format_field_section(&self, section: &str) -> Result<String, CronAnalyzerError> {
        let raw_string = section.to_owned();
        let selection = self.selection();
        let max = self.max();
        let name = self.name();

        if raw_string == "*" {
            return Ok(format!("every {}", &name));
        } else {
            let re = Regex::new(r"\d+|\w+|.").unwrap();

            let sections = re
                .find_iter(&raw_string)
                .map(|m| self.convert_if_word(m.as_str()))
                .collect::<Vec<String>>();

            let date_from_selection = |index: usize| match &selection {
                None => index.to_string(),
                Some(v) => v[index].to_string(),
            };

            match sections[0].parse::<usize>() {
                Ok(index) => {
                    self.in_range(index)?;
                    match sections.len() {
                        1 => Ok(format!(" {}", &date_from_selection(index))),
                        3 => match sections[2].parse::<usize>() {
                            Err(_) => Err(CronAnalyzerError::new(format!(
                                "Invalid input '{}' at '{}' field",
                                &section, &name
                            ))),
                            Ok(num) => match sections[1].as_str() {
                                "/" => Ok(format!(
                                    "every {} {} from {} through {}",
                                    &self.suffix(&sections[2])?,
                                    &name,
                                    &date_from_selection(index),
                                    &date_from_selection(max)
                                )),
                                "-" => {
                                    self.in_range(num)?;
                                    Ok(format!(
                                        "every {} from {} through {}",
                                        &name,
                                        &date_from_selection(index),
                                        &date_from_selection(num)
                                    ))
                                }
                                _ => Err(CronAnalyzerError::new(format!(
                                    "Invalid input '{}' at '{}' field",
                                    &section, &name
                                ))),
                            },
                        },
                        5 => {
                            let num = sections[2].parse::<usize>().unwrap();
                            self.in_range(num)?;
                            match sections[1] == "-"
                                && num >= index
                                && sections[3] == "/"
                                && sections[4].parse::<usize>().unwrap() >= 1
                            {
                                true => Ok(format!(
                                    "every {} {} from {} through {}",
                                    &self.suffix(&sections[4])?,
                                    &name,
                                    &date_from_selection(index),
                                    &date_from_selection(num)
                                )),
                                false => Err(CronAnalyzerError::new(format!(
                                    "Invalid input '{}' at '{}' field",
                                    &section, &name
                                ))),
                            }
                        }
                        _ => Err(CronAnalyzerError::new(format!(
                            "Invalid input '{}' at '{}' field",
                            &section, &name
                        ))),
                    }
                }
                Err(_) => {
                    match sections.len() == 3
                        && sections[1] == "/"
                        && sections[2].parse::<usize>().is_ok()
                        && sections[0] == "*"
                    {
                        true => Ok(format!("every {} {}", &self.suffix(&sections[2])?, &name)),
                        false => Err(CronAnalyzerError::new(format!(
                            "Invalid input '{}' at '{}' field",
                            &section, &name
                        ))),
                    }
                }
            }
        }
    }
}
