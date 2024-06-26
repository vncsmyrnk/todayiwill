use std::{fs, fs::File, io::Write};

use assert_cmd::Command;
use serial_test::serial;

fn helper_remove_data_file() {
    let appointments_path = dirs::data_dir()
        .unwrap()
        .join(String::from("todayiwill"))
        .join(String::from("appointments.txt"));
    match fs::remove_file(appointments_path) {
        _ => return,
    }
}

fn helper_write_to_data_file(content: &[u8]) {
    let base_dir = dirs::data_dir().unwrap().join(String::from("todayiwill"));
    fs::create_dir_all(base_dir.to_str().unwrap()).expect("Failed to create data dir");
    let appointments_path = base_dir.join(String::from("appointments.txt"));
    let mut file =
        File::create(appointments_path.to_str().unwrap()).expect("Failed to create test file");
    file.write_all(content)
        .expect("Failed to write to test file");
}

#[test]
#[serial]
fn empty_list() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");
}

#[test]
#[serial]
fn list_appointments() {
    helper_remove_data_file();
    helper_write_to_data_file(b"08:12 Call mom\n14:45 Listen to music\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("08:12 Call mom\n14:45 Listen to music\n");

    helper_remove_data_file();
}

#[test]
#[serial]
fn add_appointment() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "16:50"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("16:50 A certain event\n");

    helper_remove_data_file();
}

#[test]
#[serial]
fn clear_appointments() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "20:10"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("20:10 An urgent event\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["clear"])
        .assert()
        .success()
        .stdout("Appointments cleared successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    helper_remove_data_file();
}

#[test]
#[serial]
fn list_current_time() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "Clean bedroom", "--time", "19:00"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "Brush teeth", "--time", "22:30"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "10:00"])
        .assert()
        .success()
        .stdout("19:00 Clean bedroom\n22:30 Brush teeth\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:29"])
        .assert()
        .success()
        .stdout("22:30 Brush teeth\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:30"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:30", "--all"])
        .assert()
        .success()
        .stdout("19:00 Clean bedroom\n22:30 Brush teeth\n");

    helper_remove_data_file();
}

#[test]
#[ignore]
#[serial]
fn list_expire_in_x_mins() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "Schedule doctor appointment", "--time", "10:23"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "Reply to an important e-mail", "--time", "09:45"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "20"])
        .assert()
        .success()
        .stdout("09:45 Reply to an important e-mail\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "10"])
        .assert()
        .success()
        .stdout("No appointments found.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "60"])
        .assert()
        .success()
        .stdout("09:45 Reply to an important e-mail\n10:30 Schedule doctor appointment\n");
}

#[test]
fn invalid_entries() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "9:y3"])
        .assert()
        .failure()
        .code(1)
        .stdout("You entered a non-valid time.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "24:10"])
        .assert()
        .failure()
        .code(1)
        .stdout("Appointment time invalid. Hour should be between 0 and 23\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "15:60"])
        .assert()
        .failure()
        .code(1)
        .stdout("Appointment time invalid. Minutes should be between 0 and 59\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "23:60"])
        .assert()
        .failure()
        .code(2);

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "as:"])
        .assert()
        .failure()
        .code(2);
}
