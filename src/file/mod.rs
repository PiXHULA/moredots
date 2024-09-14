use std::{env, fs};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn read_from_file() -> std::io::Result<String> {
    let path = relative_path("user_setting.txt")?;
    let result = fs::read_to_string(path);
    result
}
pub fn write_to_file(content: &[u8], file_name: &str) {
    let path = relative_path(file_name);
    match File::create(path.unwrap()) {
        Ok(file) => {
            let mut write_buffer = BufWriter::new(file);
            write_buffer.write_all(content).expect("Failed to write to buffer");
            write_buffer.flush().expect("Failed to flush it");
        },
        _ => {
            eprintln!("Failed to write to file");
        }
    }
}

fn relative_path(file_name: &str) -> std::io::Result<PathBuf> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get binary directory");

    let mut path = PathBuf::from(exe_dir);
    path.push(file_name);
    Ok(path)
}