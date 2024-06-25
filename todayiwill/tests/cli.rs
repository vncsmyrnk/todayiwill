use std::{fs, fs::File, io::Write};

use assert_cmd::Command;

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
fn cli_usage() {
    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    helper_write_to_data_file(b"08:12 Call mom\n14:45 Listen to music\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("08:12 Call mom\n14:45 Listen to music\n");

    helper_remove_data_file();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "16:50"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

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
        .failure()
        .code(1)
        .stdout("You entered a non-valid time.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "20:10"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("16:50 A certain event\n20:10 An urgent event\n");

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

    helper_remove_data_file();
}
