use std::{fs::remove_file, fs::File, io::Write};

use assert_cmd::Command;

fn helper_remove_data_file() {
    let appointments_path = dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from("appointments.txt"));
    match remove_file(appointments_path) {
        _ => return,
    }
}

fn helper_write_to_data_file(content: &[u8]) {
    let appointments_path = dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from("appointments.txt"));
    let mut file =
        File::create(appointments_path.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}

#[test]
fn list() {
    helper_remove_data_file();
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("There are no appointments added for today\n");

    helper_write_to_data_file(b"14:45 Listen to music\n08:12 Call mom\n");
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("14:45 Listen to music\n08:12 Call mom\n");
    helper_remove_data_file();
}
