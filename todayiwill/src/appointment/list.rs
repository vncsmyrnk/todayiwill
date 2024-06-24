use std::{fs::read_to_string, path::PathBuf};

use crate::appointment::AppointmentTime;

use super::{Appointment, Config};

/// Displays the list of appointments in the standard output
pub fn display_list(config: Config) {
    let appointments = get_appointments_from_file(&config.appointments_path);
    if appointments.is_empty() {
        println!("There are no appointments added for today")
    }
    for appointment in &appointments {
        println!("{}", appointment)
    }
}

/// Get the string version of the list of appointments
/// Should read the appointments of a specific file and return a list
/// of appointments
fn get_appointments_from_file(path: &PathBuf) -> Vec<Appointment> {
    let file_result = read_to_string(path);
    let file_content = match file_result {
        Ok(content) => content,
        Err(..) => String::new(),
    };
    let appointments: Vec<Option<Appointment>> =
        file_content.lines().map(parse_file_line).collect();
    appointments.into_iter().flatten().collect()
}

/// Parses a string representing a file line and return an appointment
fn parse_file_line(line: &str) -> Option<Appointment> {
    let time: String = line.chars().take(5).collect();
    let (hour, minutes): (i32, i32) = time.split_once(':').and_then(|(hour_str, minute_str)| {
        let hour = hour_str.parse().ok()?;
        let minutes = minute_str.parse().ok()?;
        Some((hour, minutes))
    })?;
    let description = line.chars().skip(6).collect();
    Some(Appointment::new(
        description,
        AppointmentTime::new(hour, minutes),
    ))
}

#[cfg(test)]
mod tests {
    use std::{fs::remove_file, fs::File, io::Write, path::PathBuf};

    use crate::appointment::{
        list::{get_appointments_from_file, parse_file_line},
        Appointment, AppointmentTime,
    };

    #[test]
    fn parse_wellformed_line() {
        let result = parse_file_line("16:03 This is an example").unwrap();
        assert_eq!(
            result,
            Appointment {
                description: "This is an example".to_string(),
                time: AppointmentTime {
                    hour: 16,
                    minutes: 3
                }
            }
        );
    }

    #[test]
    fn pase_malformed_line_time_fail() {
        let result = parse_file_line("10:0 This is an incorrect example");
        assert!(result.is_none());
    }

    #[test]
    fn parse_file_contents() {
        let test_file_path = PathBuf::from("/tmp/test_file.txt");
        let mut file =
            File::create(test_file_path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(b"22:00 Go to night shift\n12:45 Visit grandma\n212 Nonsense")
            .expect("Failed to write to test file");
        let result = get_appointments_from_file(&test_file_path);
        assert_eq!(
            result,
            vec![
                Appointment::new("Go to night shift".to_string(), AppointmentTime::new(22, 0)),
                Appointment::new("Visit grandma".to_string(), AppointmentTime::new(12, 45)),
            ]
        );
        remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn parse_file_non_existent() {
        let test_file_path = PathBuf::from("/tmp/non_existent.txt");
        let result = get_appointments_from_file(&test_file_path);
        assert_eq!(result, vec![]);
    }
}