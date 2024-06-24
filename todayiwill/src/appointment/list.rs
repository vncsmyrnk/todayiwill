use crate::appointment::AppointmentTime;

use super::Appointment;

/// Displays the list of appointments in the standard output
pub fn display_list() {
    let appointments = get_appointments();
    for appointment in &appointments {
        println!("{}", appointment);
    }
}

/// Get the string version of the list of appointments
/// Should read the appointments of a specific file and return a list
/// of appointments
pub fn get_appointments() -> Vec<Appointment> {
    vec![
        Appointment::new("Do homework".to_string(), AppointmentTime::new(15, 45)),
        Appointment::new("Feed the cat".to_string(), AppointmentTime::new(8, 30)),
    ]
}

// fn read_file_content(file_path: &str) -> Result<Vec<String>, io::Error> {
//     let mut file_content = String::new();
//     File::open(file_path)?.read_to_string(&mut file_content)?;
//     let appointments: Vec<String> = file_content.lines().map(parse_file_line).collect();
//     Ok(appointments)
// }

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
    use crate::appointment::{list::parse_file_line, Appointment, AppointmentTime};

    #[test]
    fn test_parse_ok() {
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
    fn test_parse_fail() {
        let result = parse_file_line("10:0 This is an incorrect example");
        assert!(result.is_none());
    }
}
