use core::fmt;
use std::path::PathBuf;

extern crate dirs;

pub mod add;
pub mod clear;
pub mod helper;
pub mod list;

pub struct Config {
    pub appointments_path: Box<PathBuf>,
}

impl Config {
    pub fn default() -> Self {
        let base_dir = dirs::data_dir().unwrap().join("todayiwill");
        let appointments_path = base_dir.join("appointments.txt");
        Self {
            appointments_path: Box::new(appointments_path),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppointmentTime {
    pub hour: i32,
    pub minutes: i32,
}

impl AppointmentTime {
    pub fn new(hour: i32, minutes: i32) -> Result<Self, String> {
        if hour < 0 || hour > 23 {
            return Err(String::from("Hour should be between 0 and 23"));
        }
        if minutes < 0 || minutes > 59 {
            return Err(String::from("Minutes should be between 0 and 59"));
        }
        Ok(Self { hour, minutes })
    }
}

impl fmt::Display for AppointmentTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Appointment {
    pub description: String,
    pub time: AppointmentTime,
}

impl Appointment {
    pub fn new(description: String, time: AppointmentTime) -> Self {
        Self { description, time }
    }
}

impl fmt::Display for Appointment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.time, self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::AppointmentTime;

    #[test]
    fn wellformed_appointment_time() {
        let result = AppointmentTime::new(20, 34);
        assert_eq!(result.unwrap(), AppointmentTime { hour: 20, minutes: 34 });
    }

    #[test]
    fn malformed_appointment_time() {
        let result = AppointmentTime::new(26, -5);
        assert!(result.is_err());
    }
}
