use std::{fs, path::PathBuf};

use chrono::NaiveDate;

use crate::appointment::AppointmentTime;

use super::{Appointment, Config};

/// Displays the list of appointments in the standard output
pub fn display_list(ref_time: Option<AppointmentTime>, expire_time: Option<i32>, config: Config) {
    let mut appointments = get_appointments_from_file(&config.appointment_file_path_current_day);
    if appointments.is_empty() {
        println!("There are no appointments added for today.");
        return;
    }
    if ref_time.is_some() {
        let lower_limit = ref_time.unwrap();
        let upper_limit = match expire_time {
            Some(value) => lower_limit.clone() + value,
            None => AppointmentTime::max_value(),
        };
        appointments = appointments
            .into_iter()
            .filter(|a| a.time > lower_limit)
            .filter(|a| a.time <= upper_limit)
            .collect();
    }
    if appointments.is_empty() {
        println!("No appointments found.");
        return;
    }
    appointments.sort();
    for appointment in &appointments {
        println!("{}", appointment)
    }
}

/// Displays all appointments for specific dates
pub fn display_all_from(date: NaiveDate, config: Config) {
    let mut appointments =
        get_appointments_from_file(&(config.appointment_file_path_builder)(date));
    if appointments.is_empty() {
        println!("There were no appointments added in this day.");
        return;
    }
    appointments.sort();
    for appointment in &appointments {
        println!("{}", appointment)
    }
}

/// Get the string version of the list of appointments
/// Should read the appointments of a specific file and return a list
/// of appointments
pub fn get_appointments_from_file(path: &PathBuf) -> Vec<Appointment> {
    let file_result = fs::read_to_string(path);
    let file_content = match file_result {
        Ok(content) => content,
        Err(..) => String::new(),
    };
    let appointments: Vec<Result<Appointment, String>> =
        file_content.lines().map(Appointment::from).collect();
    appointments.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use std::{fs, fs::File, io::Write, path::PathBuf};

    use crate::appointment::{list::get_appointments_from_file, Appointment, AppointmentTime};

    #[test]
    fn parse_file_contents() {
        let test_file_path = PathBuf::from("/tmp")
            .join("todayilearn-test-list")
            .join("appointments.txt");
        fs::create_dir_all(test_file_path.parent().unwrap()).expect("Failed to create test dir");
        let mut file =
            File::create(test_file_path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(b"22:00 Go to night shift\n12:45 Visit grandma\n212 Nonsense")
            .expect("Failed to write to test file");
        let result = get_appointments_from_file(&test_file_path);
        assert_eq!(
            result,
            vec![
                Appointment::new(
                    "Go to night shift".to_string(),
                    AppointmentTime::new(22, 0).unwrap()
                ),
                Appointment::new(
                    "Visit grandma".to_string(),
                    AppointmentTime::new(12, 45).unwrap()
                ),
            ]
        );
        fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn parse_file_non_existent() {
        let test_file_path = PathBuf::from("/tmp/non_existent.txt");
        let result = get_appointments_from_file(&test_file_path);
        assert_eq!(result, vec![]);
    }
}
