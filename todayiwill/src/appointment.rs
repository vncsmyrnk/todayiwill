use core::fmt;
use std::path::PathBuf;

use chrono::Local;

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn now() -> Self {
        let now = Local::now();
        Self {
            hour: now.format("%H").to_string().parse().unwrap(),
            minutes: now.format("%M").to_string().parse().unwrap(),
        }
    }

    pub fn from(time: &str) -> Result<Self, String> {
        let (hour, minutes) = match helper::parse_time(time) {
            Some((hour, minutes)) => (hour, minutes),
            None => return Err(String::from("Invalid string for appointment time")),
        };
        let appointment_time = Self::new(hour, minutes)?;
        Ok(appointment_time)
    }
}

impl fmt::Display for AppointmentTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Appointment {
    pub time: AppointmentTime,
    pub description: String,
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
    use chrono::Local;

    use super::AppointmentTime;

    #[test]
    fn wellformed_appointment_time() {
        let result = AppointmentTime::new(20, 34);
        assert_eq!(
            result.unwrap(),
            AppointmentTime {
                hour: 20,
                minutes: 34
            }
        );
    }

    #[test]
    fn malformed_appointment_time() {
        let result = AppointmentTime::new(26, -5);
        assert!(result.is_err());
    }

    #[test]
    fn wellformed_appointment_time_now() {
        let time = Local::now();
        let appointment_time_now = AppointmentTime::now();
        assert_eq!(
            appointment_time_now,
            AppointmentTime::new(
                time.format("%H").to_string().parse().unwrap(),
                time.format("%M").to_string().parse().unwrap()
            )
            .unwrap()
        );
    }

    #[test]
    fn wellformed_appointment_time_from_string() {
        let appointment_time = AppointmentTime::from("10:23").unwrap();
        assert_eq!(appointment_time, AppointmentTime::new(10, 23).unwrap());
    }

    #[test]
    fn malformed_appointment_time_from_string() {
        let result = AppointmentTime::from("102y");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_appointment_time_from_string() {
        let result = AppointmentTime::from("12:76");
        assert!(result.is_err());
    }
}
