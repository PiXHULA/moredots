use std::fmt;
use crate::settings;
use std::io::{self, Write};
use std::num::ParseIntError;
use chrono::NaiveTime;

#[derive(Debug)]
pub struct Timestamp {
    pub now: NaiveTime,
    pub stop_time: NaiveTime,
    pub lunch_start_time: NaiveTime,
    pub lunch_stop_time: NaiveTime,
}

#[derive(Debug)]
pub struct NumberParseError {
    message: String,
    source: ParseIntError,
}

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.message, self.source)
    }
}

impl Timestamp {
    pub fn new() -> Self {
        let user_settings_result = settings::load_saved_settings();
        let new_timestamp = match user_settings_result {
            Ok(user_settings) => {
                Self {
                    now: Timestamp::now(),
                    stop_time: Timestamp::from(&user_settings[0]),
                    lunch_start_time: Timestamp::from(&user_settings[1]),
                    lunch_stop_time: Timestamp::from(&user_settings[2]),
                }
            }
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                Self {
                    now: Timestamp::now(),
                    stop_time: Timestamp::from(&String::from("21:00")),
                    lunch_start_time: Timestamp::from(&String::from("12:00")),
                    lunch_stop_time: Timestamp::from(&String::from("13:00")),
                }
            }
        };
        new_timestamp
    }

    fn from(string_stamp: &str) -> NaiveTime {
        let stamps: Vec<u32> = Timestamp::split_into_numbers(string_stamp);
        match stamps[..] {
            [hour] => NaiveTime::from_hms_opt(hour, 0, 0)
                .unwrap_or_default(),
            [hour, min] => NaiveTime::from_hms_opt(hour, min, 0)
                .unwrap_or_default(),
            [hour, min, sec] => NaiveTime::from_hms_opt(hour, min, sec)
                .unwrap_or_default(),
            _ => NaiveTime::default(),
        }
    }

    fn split_into_numbers<'a>(settings: &'a str) -> Vec<u32> { //returnera en Vec<u32>
        let temp_array: Vec<&'a str> = settings.split(":").collect();
        temp_array
            .clone()
            .into_iter()
            .filter_map(|part| {
                match Timestamp::parse_number_to_int(part) {
                    Ok(num) => Some(num),
                    Err(err) => {
                        eprintln!("{}", err);
                        Some(0)
                    }
                }
            })
            .collect()
    }

    fn parse_number_to_int(value: &str) -> Result<u32, NumberParseError> {
        value.parse::<u32>().map_err(|e| NumberParseError {
            message: format!("Value '{}' could not be parsed into a number. Please, what is wrong with you?", value),
            source: e,
        })
    }

    fn parse_number_to_string(value: &str) -> Result<String, NumberParseError> {
        let num = value.parse::<u32>().map_err(|e| NumberParseError {
            message: format!("Value '{}' could not be parsed into a number. Please, what is wrong with you?", value),
            source: e,
        });
        Ok(num?.to_string())
    }

    pub fn now() -> NaiveTime {
        chrono::Local::now().time()
    }
}

pub fn change_time(question: &str, stamp: &str) -> bool {
    print!("CHANGE {} TIME? {} (y/n) ", question, stamp);

    let mut input = String::new();
    io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.contains("y")
}


pub fn add_time() -> Result<String, NumberParseError> {
    print!("Enter a time (HH:MM): ");

    let mut input = String::new();
    io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately
    io::stdin().read_line(&mut input).expect("Failed to read input");

    Timestamp::parse_number_to_string(input.trim())
}