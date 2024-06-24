use std::{fs::create_dir_all, fs::remove_file, fs::File, io::Write};

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
    let base_dir = dirs::data_dir().unwrap().join(String::from("todayiwill"));
    create_dir_all(base_dir.to_str().unwrap()).expect("Failed to create data dir");
    let appointments_path = base_dir.join(String::from("appointments.txt"));
    let mut file =
        File::create(appointments_path.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}

#[test]
fn cli_usage() {
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

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "16:50"])
        .assert()
        .success()
        .stdout("Appointment added successfully\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("16:50 A certain event\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "9:y3"])
        .assert()
        .success()
        .stdout("You entered a non-valid time.\n");

    helper_remove_data_file();
}
