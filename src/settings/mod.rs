use crate::{time_handler};
use chrono::NaiveTime;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{BufWriter, Write};

struct UserSetting {
    stop_day_time: NaiveTime,
    start_lunch_time: NaiveTime,
    stop_lucnh_time: NaiveTime,
    print_timestamps: bool
}

pub fn load_saved_settings() -> Result<[String; 3]> {
    match read_from_file() {
        Ok(setting) => Ok(map_to_array(setting)),
        Err(_) => {
            let new_setting = create_settings_from_input()?;
            write_to_file(new_setting.as_bytes())?;
            Ok(map_to_array(new_setting))
        }
    }
}

fn map_to_array<'a>(value: String) -> [String; 3] {
    let temp_array: Vec<&str> = value.split(",").collect();
    temp_array
        .into_iter()
        .map(|part| part.to_string()) //Closure (anonymous function)
        .collect::<Vec<String>>()
        .try_into()
        .unwrap_or_else(|_| [  // Fallback value if conversion fails
            String::from("18:45"),
            String::from("11:00"),
            String::from("12:00")
        ])
}

fn create_settings_from_input() -> Result<String> {
    let mut timestamps = String::new();
    if time_handler::change_time("STOP", "16:45") {
        timestamps.push_str(&*time_handler::add_time().unwrap_or_else(|_| String::from("16:45")));
        timestamps.push(',');
    } else {
        timestamps.push_str("16:45");
        timestamps.push(',');
    }
    if time_handler::change_time("LUNCH", "11:00-12:00") {
        timestamps.push_str(&*time_handler::add_time().unwrap_or_else(|_| String::from("11:00")));
        timestamps.push(',');
        timestamps.push_str(&*time_handler::add_time().unwrap_or_else(|_| String::from("12:00")));
    } else {
        timestamps.push_str("11:00,12:00");
    }
    Ok(timestamps)
}


fn read_from_file() -> Result<String> {
    let result = fs::read_to_string("moredots");
    result
}

fn write_to_file(content: &[u8]) -> Result<String> {
    //TODO: Kan jag specifiera bättre var min config fil ska sparas?
    let result = File::create("moredots");
    if result.is_ok() {
        let mut write_buffer = BufWriter::new(result?);
        write_buffer.write_all(content)?;
        write_buffer.flush()?;
    }
    let read = read_from_file();
    read
}