use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use chrono::Local;

/// Provides the startup scripts for each test execution
pub fn setup() {
    helper_remove_data_file()
}

/// Clears the appointment file
pub fn helper_remove_data_file() {
    let data_file = appointments_file();
    if !data_file.exists() {
        return;
    }
    match fs::remove_file(data_file) {
        Err(error) => panic!("Failed to remove data file. {error}"),
        _ => return,
    }
}

/// Writes pre-formatted text to the appointment file
pub fn helper_write_to_data_file(content: &[u8]) {
    let data_file = appointments_file();
    fs::create_dir_all(data_file.parent().unwrap()).expect("Failed to create data dir");
    let mut file = File::create(data_file.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}

fn appointments_file() -> PathBuf {
    let current_date_code = Local::now().format("%d%m%Y").to_string();
    dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from(format!(
            "appointments_{current_date_code}.txt"
        )))
}
