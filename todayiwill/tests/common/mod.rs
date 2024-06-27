use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use chrono::{Local, NaiveDate};

/// Provides the startup scripts for each test execution
pub fn setup() {
    // helper_remove_data_file()
    remove_all_appointment_files()
}

/// Clears the appointments file
pub fn remove_all_appointment_files() {
    for entry in app_data_dir()
        .read_dir()
        .expect("Failed to access data dir")
    {
        if let Ok(entry) = entry {
            match fs::remove_file(entry.path()) {
                Err(error) => panic!("Failed to remove data file. {error}"),
                _ => return,
            }
        }
    }
}

/// Writes pre-formatted text to the appointment file
pub fn helper_write_to_appointment_data_file(content: &[u8], date: NaiveDate) {
    let data_file = appointments_file(date);
    fs::create_dir_all(data_file.parent().unwrap()).expect("Failed to create data dir");
    let mut file = File::create(data_file.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}

pub fn helper_write_to_appointment_current_day_data_file(content: &[u8]) {
    helper_write_to_appointment_data_file(content, Local::now().date_naive())
}

/// Returns the appointment file corresponding to the date
fn appointments_file(date: NaiveDate) -> PathBuf {
    let current_date_code = date.format("%d%m%Y").to_string();
    dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from(format!(
            "appointments_{current_date_code}.txt"
        )))
}

/// Returns the app data dir for testing
fn app_data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join(String::from("todayiwill"))
}
