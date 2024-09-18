use crate::file;
use std::fmt;
use std::fmt::Debug;
use std::io::{self, Write};
use std::num::ParseIntError;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::NaiveTime;

#[derive(Debug, Clone, PartialEq)]
pub enum TimestampError {
    ParseError { message: String, source: ParseIntError },
    MaxValueError { message: String },
    WrongAnswerError { message: String },
}

impl fmt::Display for TimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampError::ParseError { message, source } => write!(f, "{}:{:?}", message, source),
            TimestampError::MaxValueError { message } => write!(f, "{}", message),
            TimestampError::WrongAnswerError { message } => write!(f, "{}", message)
        }
    }
}

pub fn from(string_stamp: &str) -> NaiveTime {
    let stamps: Vec<u32> = split_into_numbers(string_stamp);
    //[12,34]
    match stamps[..] {
        [hour] => NaiveTime::from_hms_opt(hour, 0, 0)
            .unwrap_or_default(),
        [hour, min] => NaiveTime::from_hms_opt(hour, min, 0)
            .unwrap_or_default(),
        _ => NaiveTime::default(),
    }
}

fn split_into_numbers<'a>(settings: &'a str) -> Vec<u32> {
    let temp_array: Vec<&'a str> = settings.split(":").collect();
    temp_array
        .clone()
        .into_iter()
        .filter_map(|part| {
            match parse_number_to_int(part) {
                Ok(num) => Some(num),
                Err(..) => Some(0)
            }
        })
        .collect()
}

fn parse_number_to_int(value: &str) -> Result<u32, TimestampError> {
    value.parse::<u32>().map_err(|source| TimestampError::ParseError {
        message: format!("Value '{}' could not be parsed into a number with in 0-59 or 0-24. \
                          Please, what is wrong with you?", value),
        source,
    })
}

pub fn parse_number_to_string(value: &str) -> Result<String, TimestampError> {
    let stamps: Vec<&str> = value.split(":").collect();
    match stamps[..] {
        [hour] => {
            let num_hour = hour.parse::<u32>().map_err(|source| TimestampError::ParseError {
                message: format!("Value '{}' could not be parsed into a number with in 0-59 or 0-24. \
                                  Please, what is wrong with you?", hour),
                source,
            });
            match num_hour {
                Ok(0u32..=9u32) => Ok(format!("0{}:00", num_hour?.to_string())),
                Ok(10u32..24u32) => Ok(format!("{}:00", num_hour?.to_string())),
                Ok(24u32..=u32::MAX) => Err(TimestampError::MaxValueError {
                    message: format!("Value '{}' is too high. Please, what is wrong with you?", hour),
                }),
                Err(err) => Err(err)
            }
        }
        [hour, ""] => Err(TimestampError::WrongAnswerError {
            message: format!("Value '{}': is incorrect. Please, what is wrong with you?", hour),
        }),
        ["", min] => Err(TimestampError::WrongAnswerError {
            message: format!("Value :'{}' is incorrect. Please, what is wrong with you?", min),
        }),
        [hour, min] => {
            let num_hour = hour.parse::<u32>().map_err(|source| TimestampError::ParseError {
                message: format!("Value '{}' could not be parsed into hours. Please, what is wrong with you?", hour),
                source,
            });
            let result_hour = match num_hour {
                Ok(0u32..=9u32) => Ok(format!("0{}", num_hour?.to_string())),
                Ok(10u32..=23u32) => Ok(num_hour?.to_string()),
                Ok(24u32..=u32::MAX) => Err(TimestampError::MaxValueError {
                    message: format!("Value '{}' is too high. Please, what is wrong with you?", hour)
                }),
                Err(err) => Err(err)
            };
            let num_min = min.parse::<u32>().map_err(|source| TimestampError::ParseError {
                message: format!("Value '{}' could not be parsed into minutes. Please, what is wrong with you?", min),
                source,
            });
            let result_minute = match num_min {
                Ok(0u32..=9u32) => Ok(format!("{}:0{}", result_hour?.to_string(), num_min?.to_string())),
                Ok(10u32..=59u32) => Ok(format!("{}:{}", result_hour?.to_string(), num_min?.to_string())),
                Ok(60u32..=u32::MAX) => Err(TimestampError::MaxValueError {
                    message: format!("Value '{}' is too high. Please, what is wrong with you?", min),
                }),
                Err(err) => Err(err)
            };
            result_minute
        }
        _ => Ok("21:00".parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_to_int() {
        assert_eq!(parse_number_to_int("1"), Ok(1));
        assert!(parse_number_to_int("a").is_err());
    }
    #[test]
    fn test_parse_number_to_string() {
        assert_eq!(parse_number_to_string("1"), Ok(String::from("01:00")));
        assert_eq!(parse_number_to_string("1:10"), Ok(String::from("01:10")));
        assert_eq!(parse_number_to_string("10"), Ok(String::from("10:00")));
        assert_eq!(parse_number_to_string("10:1"), Ok(String::from("10:01")));
        assert_eq!(parse_number_to_string("10:10"), Ok(String::from("10:10")));

        assert!(parse_number_to_string("").is_err());
        assert!(parse_number_to_string("a").is_err());
        assert!(parse_number_to_string(":").is_err());
        assert!(parse_number_to_string("1:").is_err());
        assert!(parse_number_to_string("a:").is_err());
        assert!(parse_number_to_string(":1").is_err());
        assert!(parse_number_to_string(":a").is_err());
        assert!(parse_number_to_string("a:a").is_err());
        assert!(parse_number_to_string("10:a").is_err());
        assert!(parse_number_to_string("a:10").is_err());

        assert_eq!(parse_number_to_string("12:79"),
                   Err(TimestampError::MaxValueError {
                       message: String::from("Value '79' is too high. Please, what is wrong with you?")
                   }));
        assert_eq!(parse_number_to_string("29:30"),
                   Err(TimestampError::MaxValueError {
                       message: String::from("Value '29' is too high. Please, what is wrong with you?")
                   }))
    }
}

pub fn print(time: &NaiveTime) -> DelayedFormat<StrftimeItems> {
    time.format("%H:%M")
}

pub fn now() -> NaiveTime {
    chrono::Local::now().time()
}

pub fn change(question: &str, stamp: &str) -> bool {
    print!("Change {} time? {} (y/n) ", question, stamp);

    let mut input = String::new();
    let answer;
    loop {
        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim_end();
        let result = parse_answer_to_string(trimmed_input);
        if result.is_ok() {
            answer = result.unwrap();
            break;
        } else {
            input.clear();
            print!("Try again (y/n): ");
        }
    }
    answer
}

fn parse_answer_to_string(value: &str) -> Result<bool, TimestampError> {
    match value {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err(TimestampError::WrongAnswerError {
            message: format!("Value '{}' is not correct. Please, what is wrong with you?", value),
        }),
    }
}

pub fn add() -> String {
    print!("Enter a time (HH:MM): ");
    let mut input = String::new();
    let mut timestamp = String::new();
    loop {
        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim_end();
        let result = parse_number_to_string(trimmed_input);
        if result.is_ok() {
            timestamp.push_str(result.unwrap().trim());
            break;
        } else {
            input.clear();
            print!("Try again (HH:MM): ");
        }
    }
    println!("Time {} saved", &timestamp);
    file::write_to_file(timestamp.as_bytes(), "test_result.txt");
    String::from(timestamp.trim())
}