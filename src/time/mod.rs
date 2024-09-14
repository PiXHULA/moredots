use std::fmt;
use crate::{file};
use std::io::{self, Error, Write};
use std::num::ParseIntError;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::NaiveTime;

#[derive(Debug)]
#[derive(Clone)]
pub struct NumberParseError {
    message: String,
    source: ParseIntError,
}

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?}", self.message, self.source)
    }
}

pub fn from(string_stamp: &str) -> NaiveTime {
    let stamps: Vec<u32> = split_into_numbers(string_stamp);
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

fn split_into_numbers<'a>(settings: &'a str) -> Vec<u32> {
    let temp_array: Vec<&'a str> = settings.split(":").collect();
    temp_array
        .clone()
        .into_iter()
        .filter_map(|part| {
            match parse_number_to_int(part) {
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

//TODO: Här vill jag kunna ta in value och baserad på ifall den är i format "15" eller "15:00"
// gå igenom och se att det går att parse ordentligt
#[allow(dead_code)]
fn parse_number_to_string(value: &str) -> Result<String, NumberParseError> {
    println!("parse number: {}", value);
    let num = value.parse::<u32>().map_err(|e| NumberParseError {
        message: format!("Value '{}' could not be parsed into a number. Please, what is wrong with you?", value),
        source: e,
    });
    Ok(num?.to_string())
}

pub fn print(time: &NaiveTime) -> DelayedFormat<StrftimeItems>{
   time.format("%H:%M")
}

pub fn now() -> NaiveTime {
    chrono::Local::now().time()
}


pub fn change(question: &str, stamp: &str) -> Result<bool, Error> {
    print!("CHANGE {} TIME? {} (y/n) ", question, stamp);

    let mut input = String::new();
    io::stdout().flush()?;  // Ensure the prompt is shown immediately
    io::stdin().read_line(&mut input).expect("Failed to read input");

    Ok(input.contains("y"))
}


pub fn add() -> Result<String, Error> {
    print!("Enter a time (HH:MM): ");

    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input).expect("Failed to read input");
    file::write_to_file(input.trim().as_bytes(), "test_result.txt");
    //Går det att lägga på någon conditional match här som lägger till beroende på ifall det är 1,2 eller 3 i splitten?
    //Se @from
    Ok(String::from(input.trim()))
}