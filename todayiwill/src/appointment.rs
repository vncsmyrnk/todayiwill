use core::fmt;
use std::{ops::Add, path::PathBuf};

use chrono::{Local, NaiveDate};

extern crate dirs;

pub mod add;
pub mod clear;
pub mod helper;
pub mod list;

pub struct Config {
    pub appointment_file_path_current_day: Box<PathBuf>,
    pub appointment_file_path_builder: Box<dyn Fn(NaiveDate) -> PathBuf>,
}

impl Config {
    pub fn standard() -> Self {
        let appointment_path_builder = |date: NaiveDate| {
            dirs::data_dir()
                .unwrap()
                .join("todayiwill")
                .join(format!("appointments_{}.txt", helper::date_code(date)))
        };
        Self {
            appointment_file_path_current_day: Box::new(appointment_path_builder(
                Local::now().date_naive(),
            )),
            appointment_file_path_builder: Box::new(appointment_path_builder),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppointmentTime {
    pub hour: i32,
    pub minutes: i32,
}

impl<'a> AppointmentTime {
    pub fn new(hour: i32, minutes: i32) -> Result<Self, &'a str> {
        if !(0..24).contains(&hour) {
            return Err("Hour should be between 0 and 23");
        }
        if !(0..60).contains(&minutes) {
            return Err("Minutes should be between 0 and 59");
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

    pub fn from(time: &str) -> Result<Self, &'a str> {
        let (hour, minutes) = match helper::parse_time(time) {
            Some((hour, minutes)) => (hour, minutes),
            None => return Err("Invalid string for appointment time"),
        };
        let appointment_time = Self::new(hour, minutes)?;
        Ok(appointment_time)
    }

    pub fn max_value() -> Self {
        Self {
            hour: 23,
            minutes: 59,
        }
    }
}

impl Add<i32> for AppointmentTime {
    type Output = AppointmentTime;

    fn add(self, rhs: i32) -> Self::Output {
        let minutes_updated = self.minutes + rhs;
        let hours_updated = self.hour + (minutes_updated / 60);
        if hours_updated > 23 {
            return AppointmentTime::max_value();
        }
        Self {
            hour: hours_updated,
            minutes: minutes_updated % 60,
        }
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
    use chrono::{Local, NaiveDate};

    use super::{AppointmentTime, Config};

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
    fn malformed_appointment_time_edge_case_minutes() {
        let result = AppointmentTime::new(26, 60);
        assert!(result.is_err());
    }

    #[test]
    fn malformed_appointment_time_edge_case_hour() {
        let result = AppointmentTime::new(24, 5);
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

    #[test]
    fn invalid_appointment_time_from_string_edge_case_minutes() {
        let result = AppointmentTime::from("01:60");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_appointment_time_from_string_edge_case_hour() {
        let result = AppointmentTime::from("24:43");
        assert!(result.is_err());
    }

    #[test]
    fn max_value() {
        let result = AppointmentTime::max_value();
        assert_eq!(result, AppointmentTime::new(23, 59).unwrap());
    }

    #[test]
    fn add_i32_to_appointment_time() {
        let result = AppointmentTime::new(10, 30).unwrap() + 10;
        assert_eq!(result, AppointmentTime::new(10, 40).unwrap());
    }

    #[test]
    fn add_i32_to_appointment_time_edge_case() {
        let result = AppointmentTime::new(3, 50).unwrap() + 100;
        assert_eq!(result, AppointmentTime::new(5, 30).unwrap());
    }

    #[test]
    fn add_i32_to_appointment_time_upper_limit() {
        let result = AppointmentTime::new(23, 55).unwrap() + 20;
        assert_eq!(result, AppointmentTime::max_value());
    }

    #[test]
    fn add_i32_to_appointment_time_upper_limit_edge_case() {
        let result = AppointmentTime::new(23, 55).unwrap() + 4;
        assert_eq!(result, AppointmentTime::max_value());
    }

    #[test]
    fn config_default_should_return_a_builder_fn() {
        let result = (Config::standard().appointment_file_path_builder)(
            NaiveDate::from_ymd_opt(2023, 10, 21).unwrap(),
        );
        assert_eq!(
            result,
            dirs::data_dir()
                .unwrap()
                .join("todayiwill")
                .join("appointments_21102023.txt")
        );
    }
}
