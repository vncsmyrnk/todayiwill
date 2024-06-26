use std::{fs, fs::File, io::Write};

/// Provides the startup scripts for each test execution
pub fn setup() {
    helper_remove_data_file()
}

/// Clears the appointment file
pub fn helper_remove_data_file() {
    let appointments_path = dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from("appointments.txt"));
    match fs::remove_file(appointments_path) {
        _ => return,
    }
}

/// Writes pre-formatted text to the appointment file
pub fn helper_write_to_data_file(content: &[u8]) {
    let base_dir = dirs::data_dir().unwrap().join(String::from("todayiwill"));
    fs::create_dir_all(base_dir.to_str().unwrap()).expect("Failed to create data dir");
    let appointments_path = base_dir.join(String::from("appointments.txt"));
    let mut file =
        File::create(appointments_path.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}
