// use std::{fs::File, io::{self, Read}, path::Path};

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

// fn read_file_content(file_path: &str) -> Result<String, io::Error> {
//     let mut file_content = String::new();
//     File::open(file_path)?.read_to_string(&mut file_content)?;
//     Ok(file_content)
// }
//
// fn parse_file_content(file_content: String) -> Vec<String> {
//     let appointment = file_content.lines().map(|item: &str| { item.to_string() }).collect();
//     appointment
// }
