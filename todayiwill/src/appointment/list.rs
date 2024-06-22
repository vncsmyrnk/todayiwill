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
