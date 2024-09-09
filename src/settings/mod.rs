use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Result;
use crate::time_util;


pub fn get_setting() -> [String; 3] {
    let saved_setting = read_from_file();
    if saved_setting.is_ok() {
        convert_to_array(saved_setting.unwrap())
    } else {
        convert_to_array(get_settings_from_user())
    }
}

fn get_settings_from_user() -> String {
    let mut stamps = String::new();
    if time_util::change_time("STOP", "16:45") {
        stamps += &*time_util::add_time();
        stamps += ",";
    }
    if time_util::change_time("LUNCH", "11:00-13:00") {
        stamps += &*time_util::add_time();
        stamps += ",";
        stamps += &*time_util::add_time();
    }
    create_setting(stamps.as_bytes());
    stamps
}

fn create_setting(content: &[u8]) {
    //TODO: Kan jag specifiera bättre var min config fil ska sparas?
    if content.is_empty() {
        write_to_file(b"16:45,11:00,13:00").unwrap();
    } else {
        write_to_file(content).unwrap();
    }
}

fn convert_to_array<'a>(value: String) -> [String; 3] {
    let temp_array: Vec<&str> = value.split(",").collect();
    let mut values: [String; 3] = [String::from("16:45"), String::from("11:00"), String::from("13:00")];
    let mut counter = 0;
    for part in temp_array {
        values[counter] = part.parse().unwrap();
        counter += 1;
    }
    values
}


fn read_from_file() -> Result<String> {
    let result = fs::read_to_string("moredots");
    result
}

fn write_to_file(content: &[u8]) -> Result<String> {
    let result = File::create("moredots");
    if result.is_ok() {
        let mut write_buffer = BufWriter::new(result?);
        write_buffer.write_all(content)?;
        write_buffer.flush()?;
    }
    let read = read_from_file();
    read
}