use crate::{file, time};
use chrono::NaiveTime;
use std::io::Result;
use crate::time::add;

#[derive(Debug)]
pub struct Settings {
    pub now: NaiveTime,
    pub stop_time: NaiveTime,
    pub lunch_start_time: NaiveTime,
    pub lunch_stop_time: NaiveTime,
}

impl Settings {
    fn new(timestamps: &str) -> Self {
        let timestamps = Self::map_to_timestamp_array(timestamps);
        let new_setting = Self {
            now: time::now(),
            stop_time: timestamps[0],
            lunch_start_time: timestamps[1],
            lunch_stop_time: timestamps[2],
        };
        new_setting
    }


    pub fn save_settings(content: &str, filename: &str) {
        file::write_to_file(content.as_bytes(), filename);
    }

    fn create_settings_from_input() -> Result<String> {
        let mut timestamps = String::new();
        if time::change("stop", "16:45") {
            let result = add();
            timestamps.push_str(&format!("{},",&result));
        } else {
            timestamps.push_str("16:45,");
        }
        if time::change("lunch", "11:00-12:00") {
            let lunch_start = add();
            let lunch_stop = add();
            timestamps.push_str(&format!("{},{}", lunch_start, lunch_stop));
        } else {
            timestamps.push_str("11:00,12:00");
        }
        Ok(timestamps)
    }

    pub fn print_welcome_message(settings: &Settings) {
        let Settings { stop_time, lunch_start_time, lunch_stop_time, .. } = settings;
        println!("┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐");
        println!("| PAUSE SHIFT+P        |");
        println!("| END   ESCAPE         |");
        println!("| STOP  {}          |", time::print(stop_time));
        println!("| LUNCH {}-{}    |", time::print(lunch_start_time), time::print(lunch_stop_time));
        println!("└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘");
    }

    fn map_to_timestamp_array(value: &str) -> [NaiveTime; 3] {
        let temp_array: Vec<&str> = value.split(",").collect();
        temp_array
            .into_iter()
            .map(|part| time::from(part)) //Closure (anonymous function)
            .collect::<Vec<NaiveTime>>()
            .try_into()
            .unwrap_or_else(|_| [  // Fallback value if conversion fails
                time::from("16:45"),
                time::from("11:00"),
                time::from("12:00")
            ])
    }
}

pub fn load_settings() -> Result<Settings> {
    match file::read_from_file() {
        Ok(setting) => {
            let settings = Settings::new(&setting);
            Ok(settings)
        }
        Err(_) => {
            let input_setting = Settings::create_settings_from_input()?;
            Settings::save_settings(&input_setting, "user_setting.txt");
            let settings = Settings::new(&input_setting);
            Settings::print_welcome_message(&settings);
            Ok(settings)
        }
    }
}