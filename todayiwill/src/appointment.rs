use chrono::Local;
use core::fmt;
use std::{
    ops::{Add, Sub},
    str,
};

extern crate dirs;

pub mod helper;
pub mod list;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppointmentTime {
    pub hour: i32,
    pub minutes: i32,
}

impl AppointmentTime {
    pub fn new(hour: i32, minutes: i32) -> Result<Self, String> {
        if !(0..24).contains(&hour) {
            return Err(String::from("Hour should be between 0 and 23"));
        }
        if !(0..60).contains(&minutes) {
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

    pub fn is_past_from(&self, appointment_time: &AppointmentTime) -> bool {
        self < appointment_time
    }

    pub fn from(time: &str) -> Result<Self, String> {
        let (hour, minutes) = match helper::parse_time(time) {
            Some((hour, minutes)) => (hour, minutes),
            None => return Err(String::from("Invalid string for appointment time")),
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

    pub fn min_value() -> Self {
        Self {
            hour: 0,
            minutes: 0,
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

impl Sub<i32> for AppointmentTime {
    type Output = AppointmentTime;

    fn sub(self, rhs: i32) -> Self::Output {
        let minutes_updated = self.minutes - rhs;
        let hours_updated =
            self.hour + (minutes_updated / 60) - if minutes_updated.is_negative() { 1 } else { 0 };
        if hours_updated < 0 {
            return AppointmentTime::min_value();
        }
        Self {
            hour: hours_updated,
            minutes: minutes_updated.abs() % 60,
        }
    }
}

impl fmt::Display for AppointmentTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
    }
}

impl str::FromStr for AppointmentTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AppointmentTime::from(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Appointment {
    pub time: AppointmentTime,
    pub description: String,
}

impl Appointment {
    pub fn new(description: String, time: AppointmentTime) -> Self {
        Self { description, time }
    }

    pub fn from(appointment: &str) -> Result<Self, String> {
        let time: String = appointment.chars().take(5).collect();
        let appointment_time = AppointmentTime::from(&time)?;
        let description = appointment.chars().skip(6).collect();
        Ok(Appointment::new(description, appointment_time))
    }

    pub fn is_past_from(&self, appointment_time: &AppointmentTime) -> bool {
        self.time.is_past_from(appointment_time)
    }

    pub fn to_string_display(&self, ref_time: &AppointmentTime) -> String {
        let display = format!("[{}] {}", self.time, self.description);
        if self.is_past_from(ref_time) {
            // display.strikethrough().to_string()
            display
        } else {
            display
        }
    }
}

impl fmt::Display for Appointment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.time, self.description)
    }
}

impl str::FromStr for Appointment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Appointment::from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{Appointment, AppointmentTime};
    use chrono::Local;

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
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
    }

    #[test]
    fn malformed_appointment_time_edge_case_minutes() {
        let result = AppointmentTime::new(26, 60);
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
    }

    #[test]
    fn malformed_appointment_time_edge_case_hour() {
        let result = AppointmentTime::new(24, 5);
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
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
        let err = result.err();
        assert_eq!("Invalid string for appointment time", err.unwrap());
    }

    #[test]
    fn invalid_appointment_time_from_string() {
        let result = AppointmentTime::from("12:76");
        let err = result.err();
        assert_eq!("Minutes should be between 0 and 59", err.unwrap());
    }

    #[test]
    fn invalid_appointment_time_from_string_edge_case_minutes() {
        let result = AppointmentTime::from("01:60");
        let err = result.err();
        assert_eq!("Minutes should be between 0 and 59", err.unwrap());
    }

    #[test]
    fn invalid_appointment_time_from_string_edge_case_hour() {
        let result = AppointmentTime::from("24:43");
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
    }

    #[test]
    fn max_value() {
        let result = AppointmentTime::max_value();
        assert_eq!(result, AppointmentTime::new(23, 59).unwrap());
    }

    #[test]
    fn min_value() {
        let result = AppointmentTime::min_value();
        assert_eq!(result, AppointmentTime::new(0, 0).unwrap());
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
    fn sub_i32_to_appointment_time() {
        let result = AppointmentTime::new(10, 30).unwrap() - 20;
        assert_eq!(result, AppointmentTime::new(10, 10).unwrap());
    }

    #[test]
    fn sub_i32_to_appointment_time_edge_case() {
        let result = AppointmentTime::new(3, 10).unwrap() - 100;
        assert_eq!(result, AppointmentTime::new(1, 30).unwrap());
    }

    #[test]
    fn sub_i32_to_appointment_time_lower_limit() {
        let result = AppointmentTime::new(0, 5).unwrap() - 20;
        assert_eq!(result, AppointmentTime::min_value());
    }

    #[test]
    fn sub_i32_to_appointment_time_lower_limit_edge_case() {
        let result = AppointmentTime::new(0, 5).unwrap() - 5;
        assert_eq!(result, AppointmentTime::min_value());
    }

    #[test]
    fn create_appointment_from_str() {
        let result = Appointment::from("05:06 Take the bus");
        assert_eq!(
            result.unwrap(),
            Appointment {
                description: String::from("Take the bus"),
                time: AppointmentTime::new(5, 6).unwrap()
            }
        );
    }

    #[test]
    fn create_appointment_from_str_edge_case() {
        let result = Appointment::from("23:59 A very late appointment");
        assert_eq!(
            result.unwrap(),
            Appointment {
                description: String::from("A very late appointment"),
                time: AppointmentTime::new(23, 59).unwrap()
            }
        );
    }

    #[test]
    fn create_appointment_from_str_malformed() {
        let result = Appointment::from("16:5Fix plumbing problem");
        let err = result.err();
        assert_eq!("Invalid string for appointment time", err.unwrap());
    }

    #[test]
    fn create_appointment_from_str_malformed_without_time() {
        let result = Appointment::from("An appointment without time");
        let err = result.err();
        assert_eq!("Invalid string for appointment time", err.unwrap());
    }

    #[test]
    fn create_appointment_from_str_invalid_time() {
        let result = Appointment::from("79:81 An impossible appointment");
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
    }

    #[test]
    fn create_appointment_from_str_invalid_time_edge_case() {
        let result = Appointment::from("24:00 An impossible appointment");
        let err = result.err();
        assert_eq!("Hour should be between 0 and 23", err.unwrap());
    }

    #[test]
    fn display_appointment() {
        let appointment = Appointment::new(
            String::from("Go to the dentist"),
            AppointmentTime::new(2, 30).unwrap(),
        );
        let ref_time = AppointmentTime::new(1, 0).unwrap();
        assert_eq!(
            "[02:30] Go to the dentist",
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn display_past_appointment() {
        let appointment = Appointment::new(
            String::from("Do the laundry"),
            AppointmentTime::new(18, 5).unwrap(),
        );
        let ref_time = AppointmentTime::new(18, 0).unwrap();
        assert_eq!(
            "[18:05] Do the laundry",
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn appointment_time_should_be_passed() {
        let future_appointment_time = AppointmentTime::now() + 5;
        assert!(!future_appointment_time.is_past_from(&AppointmentTime::now()))
    }

    #[test]
    fn appointment_time_should_not_be_passed() {
        let future_appointment_time = AppointmentTime::now() - 5;
        assert!(future_appointment_time.is_past_from(&AppointmentTime::now()))
    }

    #[test]
    fn appointment_time_should_not_be_passed_edge_case() {
        let future_appointment_time = AppointmentTime::now();
        assert!(!future_appointment_time.is_past_from(&AppointmentTime::now()))
    }

    #[test]
    fn appointment_should_be_passed() {
        let future_appointment = Appointment::new(
            String::from("Some future appointment"),
            AppointmentTime::now() + 5,
        );
        assert!(!future_appointment.is_past_from(&AppointmentTime::now()))
    }

    #[test]
    fn appointment_not_should_be_passed() {
        let future_appointment = Appointment::new(
            String::from("Some past appointment"),
            AppointmentTime::now() - 5,
        );
        assert!(future_appointment.is_past_from(&AppointmentTime::now()))
    }
}
