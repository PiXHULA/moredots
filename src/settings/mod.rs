use crate::{time_handler};
use std::{env, fs};
use std::fs::File;
use std::io::Result;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn load_saved_settings() -> Result<[String; 3]> {
    match read_from_file() {
        Ok(setting) => Ok(map_to_array(setting)),
        Err(_) => {
            let new_setting = create_settings_from_input()?;
            write_to_file(new_setting.as_bytes())?;
            let settings = map_to_array(new_setting);
            print_stamps(&settings);
            Ok(settings)
        }
    }
}

fn print_stamps(settings: &[String; 3]) {
    println!("STOP  : {}", settings[0]);
    println!("LUNCH : {}-{}", settings[1], settings[2]);
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
    let path = get_relative_path_to_file()?;
    let result = fs::read_to_string(path);
    result
}

fn write_to_file(content: &[u8]) -> Result<String> {
    let path = get_relative_path_to_file()?;
    let result = File::create(path);

    if result.is_ok() {
        let mut write_buffer = BufWriter::new(result?);
        write_buffer.write_all(content)?;
        write_buffer.flush()?;
    }
    let read = read_from_file();
    read
}

fn get_relative_path_to_file() -> Result<PathBuf> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get binary directory");

    let mut path = PathBuf::from(exe_dir);
    path.push("user_settings.txt");
    Ok(path)
}