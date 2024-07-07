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
            r#"error: invalid value '9:y3' for '--time <HH:MM>': Invalid string for appointment time

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
            r#"error: invalid value '24:10' for '--time <HH:MM>': Hour should be between 0 and 23

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
            r#"error: invalid value '15:60' for '--time <HH:MM>': Minutes should be between 0 and 59

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
        .stderr(r#"error: invalid value '23:60' for '--current-time <HH:MM>': Minutes should be between 0 and 59

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
        .stderr(r#"error: invalid value '10:00pm' for '--current-time <HH:MM>': Invalid string for appointment time

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
  --description <STRING>

Usage: todayiwill add --time <HH:MM> --description <STRING>

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
  --time <HH:MM>

Usage: todayiwill add --description <STRING> --time <HH:MM>

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
  --description <STRING>
  --time <HH:MM>

Usage: todayiwill add --description <STRING> --time <HH:MM>

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
        .stderr(r#"error: invalid value '23:60' for '--current-time <HH:MM>': Minutes should be between 0 and 59

For more information, try '--help'.
"#);

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--current-time", "as:"])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value 'as:' for '--current-time <HH:MM>': Invalid string for appointment time

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
  --date <DD/MM/YYYY>

Usage: todayiwill history --date <DD/MM/YYYY>

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["history", "--date", "01-2023-22"])
        .assert()
        .failure()
        .code(2)
        .stderr(r#"error: invalid value '01-2023-22' for '--date <DD/MM/YYYY>': input contains invalid characters

For more information, try '--help'.
"#);
}

#[test]
#[serial]
fn add_from_stdin_should_be_possible() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--current-time", "19:49", "--stdin"])
        .write_stdin("20:46 Finish final assingment")
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--current-time", "03:12", "--stdin"])
        .write_stdin("16:23 Read another chapter of moby dick")
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
fn add_from_stdin_should_validate_current_time() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--current-time", "15:26", "--stdin"])
        .write_stdin("12:06 A past non-urgent event")
        .assert()
        .failure()
        .code(1)
        .stderr("Given time already passed.\n");
}

#[test]
#[serial]
fn add_from_stdin_should_not_be_run_with_other_add_args() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--current-time",
            "20:05",
            "--stdin",
            "--description",
            "Some other description",
        ])
        .write_stdin("22:46 Appointment from stdin")
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the argument '--stdin' cannot be used with '--description <STRING>'

Usage: todayiwill add --current-time <HH:MM> --stdin

For more information, try '--help'.
"#,
        );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--current-time",
            "04:45",
            "--stdin",
            "--time",
            "10:56",
        ])
        .write_stdin("11:39 Appointment from stdin")
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the argument '--stdin' cannot be used with '--time <HH:MM>'

Usage: todayiwill add --current-time <HH:MM> --stdin

For more information, try '--help'.
"#,
        );
}

#[test]
#[serial]
fn add_from_stdin_should_error_on_invalid_entries() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--stdin"])
        .write_stdin("1204 A malformed appointment")
        .assert()
        .failure()
        .code(1)
        .stderr("Invalid string for appointment time\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["add", "--stdin"])
        .write_stdin("Unformatted 10:34 appointment")
        .assert()
        .failure()
        .code(1)
        .stderr("Invalid string for appointment time\n");
}

#[test]
#[serial]
fn copy_appointments_from_past_days_should_be_ok() {
    common::setup();

    common::helper_write_to_appointment_data_file(
        b"02:55 Visit Jane on the Hospital\n08:23 Work out",
        NaiveDate::from_ymd_opt(2024, 2, 10).unwrap(),
    );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["copy", "--from", "10/02/2024"])
        .assert()
        .success()
        .stdout("Appointments copied to current day.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all", "--current-time", "01:56"])
        .assert()
        .success()
        .stdout("[02:55] Visit Jane on the Hospital\n[08:23] Work out\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn copy_appointments_from_empty_days_should_error() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("There are no appointments added for today.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["copy", "--from", "23/09/2007"])
        .assert()
        .failure()
        .code(1)
        .stderr("Given day has no appointments.\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
fn copy_command_with_no_arguments_should_error() {
    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["copy"])
        .write_stdin("11:39 Appointment from stdin")
        .assert()
        .failure()
        .code(2)
        .stderr(
            r#"error: the following required arguments were not provided:
  --from <DD/MM/YYYY>

Usage: todayiwill copy --from <DD/MM/YYYY>

For more information, try '--help'.
"#,
        );
}

#[test]
#[serial]
fn copy_command_should_error_if_there_are_appointments_for_today() {
    common::setup();

    common::helper_write_to_appointment_data_file(
        b"18:37 Buy groceries",
        NaiveDate::from_ymd_opt(2023, 8, 28).unwrap(),
    );

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Clean the house",
            "--time",
            "10:44",
            "--current-time",
            "03:12",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["copy", "--from", "28/08/2023"])
        .assert()
        .failure()
        .code(1)
        .stderr("Copy not possible, there are appointments for the current day.\n");

    common::remove_all_appointment_files();
}

#[test]
#[serial]
#[ignore = "feature of override appointments not available"]
fn add_with_an_existing_time_should_override() {
    common::setup();

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Watch the soccer game",
            "--time",
            "18:25",
            "--current-time",
            "10:00",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Search for a new car",
            "--time",
            "20:03",
            "--current-time",
            "10:01",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[18:25] Watch the soccer game\n[20:03] Search for a new car\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Learn Rust",
            "--time",
            "20:03",
            "--current-time",
            "10:01",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[18:25] Watch the soccer game\n[20:03] Learn Rust\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args([
            "add",
            "--description",
            "Clean bedsheets",
            "--time",
            "12:04",
            "--current-time",
            "10:01",
        ])
        .assert()
        .success()
        .stdout("Appointment added successfully.\n");

    Command::cargo_bin("todayiwill")
        .unwrap()
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("[12:04] Clean bedsheets\n[18:25] Watch the soccer game\n[20:03] Learn Rust\n");

    common::remove_all_appointment_files();
}
