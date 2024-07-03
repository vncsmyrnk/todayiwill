use assert_cmd::Command;
use chrono::{Local, NaiveDate};
use serial_test::serial;

mod common;

#[test]
#[serial]
fn empty_list() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn list_appointments() {
    common::setup();
    common::helper_write_to_appointment_current_day_data_file(
        b"08:12 Call mom\n14:45 Listen to music\n",
    );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[08:12] Call mom\n[14:45] Listen to music\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn add_appointment() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "A certain event",
            "--time",
            "16:50",
            "--current-time",
            "10:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[16:50] A certain event\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn clear_appointments() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "An urgent event",
            "--time",
            "20:10",
            "--current-time",
            "18:32",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[20:10] An urgent event\n");

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

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn list_current_time() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Clean bedroom",
            "--time",
            "19:00",
            "--current-time",
            "08:56",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Brush teeth",
            "--time",
            "22:30",
            "--current-time",
            "19:10",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "10:00"])
        .assert()
        .success()
        .stdout("[19:00] Clean bedroom\n[22:30] Brush teeth\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:29"])
        .assert()
        .success()
        .stdout("[22:30] Brush teeth\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:30"])
        .assert()
        .success()
        .stdout("No appointments found.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "22:30", "--all"])
        .assert()
        .success()
        .stdout("[19:00] Clean bedroom\n[22:30] Brush teeth\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn list_expire_in_x_mins() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Schedule doctor appointment",
            "--time",
            "10:23",
            "--current-time",
            "09:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Reply to an important e-mail",
            "--time",
            "09:45",
            "--current-time",
            "09:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "20"])
        .assert()
        .success()
        .stdout("[09:45] Reply to an important e-mail\n");

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
        .stdout("[09:45] Reply to an important e-mail\n[10:23] Schedule doctor appointment\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "15"])
        .assert()
        .success()
        .stdout("[09:45] Reply to an important e-mail\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30", "--expire-in", "14"])
        .assert()
        .success()
        .stdout("No appointments found.\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn add_invalid_entries_for_time() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "A certain event", "--time", "9:y3"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: invalid value '9:y3' for '--time <TIME>': Invalid string for appointment time

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "24:10"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: invalid value '24:10' for '--time <TIME>': Hour should be between 0 and 23

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "An urgent event", "--time", "15:60"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: invalid value '15:60' for '--time <TIME>': Minutes should be between 0 and 59

For more information, try '--help'.
"#,
        );
}

#[test]
#[serial]
fn add_invalid_entries_for_current_time() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Drive Martha to school",
            "--time",
            "15:30",
            "--current-time",
            "19:01",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr("Given time already passed.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Buy groceries",
            "--time",
            "11:40",
            "--current-time",
            "11:40",
        ])
        .assert()
        .failure()
        .code(1)
        .stderr("Given time already passed.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "A reminder",
            "--time",
            "14:32",
            "--current-time",
            "23:60",
        ])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '23:60' for '--current-time <CURRENT_TIME>': Minutes should be between 0 and 59

For more information, try '--help'.
"#);

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Another reminder",
            "--time",
            "15:43",
            "--current-time",
            "10:00pm",
        ])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '10:00pm' for '--current-time <CURRENT_TIME>': Invalid string for appointment time

For more information, try '--help'.
"#);
}

#[test]
#[serial]
fn add_invalid_entries_missing_parameters() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--time", "22:03"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the following required arguments were not provided:
  --description <DESCRIPTION>

Usage: todayiwill add --time <TIME> --description <DESCRIPTION>

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--description", "Wash the kitchen floor"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the following required arguments were not provided:
  --time <TIME>

Usage: todayiwill add --description <DESCRIPTION> --time <TIME>

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the following required arguments were not provided:
  --description <DESCRIPTION>
  --time <TIME>

Usage: todayiwill add --description <DESCRIPTION> --time <TIME>

For more information, try '--help'.
"#,
        );
}

#[test]
#[serial]
fn list_invalid_entries_current_time() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "23:60"])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '23:60' for '--current-time <CURRENT_TIME>': Minutes should be between 0 and 59

For more information, try '--help'.
"#);

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "as:"])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value 'as:' for '--current-time <CURRENT_TIME>': Invalid string for appointment time

For more information, try '--help'.
"#);
}

#[test]
#[serial]
fn appointments_stored_using_determined_file_name() {
    common::setup();

    let current_date = Local::now().format("%d%m%Y").to_string();
    let appointments_file = dirs::data_dir()
        .unwrap()
        .join("todayiwill")
        .join(format!("appointments_{current_date}.txt"));
    assert!(
        !appointments_file.exists(),
        "File \"{}\" exists when it should not",
        appointments_file.to_str().unwrap()
    );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Check sink problem",
            "--time",
            "09:56",
            "--current-time",
            "09:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    assert!(appointments_file.exists());

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn appointment_history() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Work on my art portfolio",
            "--time",
            "18:40",
            "--current-time",
            "09:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "history",
            "--date",
            Local::now().format("%d/%m/%Y").to_string().as_str(),
        ])
        .assert()
        .success()
        .stdout("[18:40] Work on my art portfolio\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["history", "--date", "01/01/2024"])
        .assert()
        .success()
        .stdout("There were no appointments added in this day.\n");

    common::helper_write_to_appointment_data_file(
        b"13:12 An appointment added on 01/01/2024",
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["history", "--date", "01/01/2024"])
        .assert()
        .success()
        .stdout("[13:12] An appointment added on 01/01/2024\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn history_invalid_entries() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["history"])
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the following required arguments were not provided:
  --date <DATE>

Usage: todayiwill history --date <DATE>

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["history", "--date", "01-2023-22"])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '01-2023-22' for '--date <DATE>': input contains invalid characters

For more information, try '--help'.
"#);
}

#[test]
#[serial]
fn add_from_stdin_should_be_possible() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--stdin", "20:46 Finish final assingment"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--stdin", "16:23 Read another chapter of moby dick"])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "09:30"])
        .assert()
        .success()
        .stdout("[16:23] Read another chapter of moby dick\n[20:46] Finish final assingment\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn add_from_stdin_should_error_on_invalid_entries() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--stdin",
            "1204 A malformed appointment",
        ])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '1204 A malformed appointment' for '--stdin <STDIN>': Invalid string for appointment time

For more information, try '--help'.
"#);

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--stdin",
            "Unformatted 10:34 appointment",
        ])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value 'Unformatted 10:34 appointment' for '--stdin <STDIN>': Invalid string for appointment time

For more information, try '--help'.
"#);
}
