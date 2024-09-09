use std::io::{self, Write};
use chrono::NaiveTime;
use crate::settings;

pub struct Timestamp {
    pub now: NaiveTime,
    pub stop_time: NaiveTime,
    pub lunch_start_time: NaiveTime,
    pub lunch_stop_time: NaiveTime,
}

impl Timestamp {
    pub fn new() -> Self {
        let user_settings = settings::get_setting();
        let stop: Vec<&str> = Timestamp::map_string_into_vec(&user_settings[0]);
        let start_lunch: Vec<&str> = Timestamp::map_string_into_vec(&user_settings[1]);
        let stop_lunch: Vec<&str> = Timestamp::map_string_into_vec(&user_settings[2]);
        let new_timestamp = Self {
            now: Timestamp::now(),
            stop_time: Timestamp::get_timestamp_from_string(stop),
            lunch_start_time: Timestamp::get_timestamp_from_string(start_lunch),
            lunch_stop_time: Timestamp::get_timestamp_from_string(stop_lunch),
        };
        new_timestamp.print_stamps();
        new_timestamp
    }

    fn map_string_into_vec<'a>(settings: &'a String) -> Vec<&'a str> {
        let arr: Vec<&'a str> = settings.split(":").collect();
        arr
    }

    pub fn print_stamps(&self) {
        println!("STOP TIME  : {}", self.stop_time.format("%H:%M"));
        println!("LUNCH TIME : {}-{}", self.lunch_start_time.format("%H:%M"), self.lunch_stop_time.format("%H:%M"));
    }

    fn get_timestamp_from_string(timestamp: Vec<&str>) -> NaiveTime {
        let mut hour= "0";
        let mut min= "0";
        if timestamp.len() == 1 {
            hour = timestamp[0];
        }
        if timestamp.len() == 2 {
            hour = timestamp[0];
            min = timestamp[1];
        }
        NaiveTime::from_hms_opt(Timestamp::parse_string(hour), Timestamp::parse_string(min), 0).unwrap()
    }

    fn parse_string(value: &str) -> u32 {
        value.parse::<u32>().unwrap_or_else(|e| {
            eprintln!("Value {} could not be parse -> Error {:?}", value, e);
            00u32
        })
    }


    pub fn now() -> NaiveTime {
        chrono::Local::now().time()
    }


    pub fn time_from_input() -> NaiveTime {
        // Ask the user to input a time
        print!("Enter a time (HH:MM): ");
        io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Trim the newline character
        let input = input.trim();

        // Parse the input into a NaiveTime
        let parsed_time = NaiveTime::parse_from_str(input, "%H:%M");

        // Check if the input was valid
        match parsed_time {
            Ok(time) => {
                // Add 30 minutes to the parsed time
                println!("New time: {}", time.format("%H:%M"));
                parsed_time.unwrap()
            }
            _ => {
                // Handle invalid input
                eprintln!("{:?}. Please use HH:MM format.", parsed_time);
                NaiveTime::from_hms_opt(00, 00, 00).unwrap()
            }
        }
    }
}

pub fn change_time(question: &str, stamp: &str) -> bool {
    print!("CHANGE {} TIME? {} (y/n) ", question, stamp);
    io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately

    // Get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.contains("y")
}


pub fn add_time() -> String {
    // Ask the user to input a time
    print!("Enter a time (HH:MM): ");
    io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately

    // Get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the newline character
    let input = input.trim();

    String::from(input)
}