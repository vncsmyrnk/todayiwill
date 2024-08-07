use colored::Colorize;
use core::fmt;
use std::str;

extern crate dirs;

pub mod helper;
pub mod list;
pub mod time;

use time::AppointmentTime;

/// Describe an appointment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Appointment {
    time: AppointmentTime,
    description: String,
}

impl Appointment {
    /// Returns a new `Appointment`
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentTime};
    ///
    /// let appointment = Appointment::new(String::from("A description"), AppointmentTime::new(9, 30).unwrap());
    /// assert_eq!("09:30 A description", appointment.to_string());
    /// ```
    pub fn new(description: String, time: AppointmentTime) -> Self {
        Self { description, time }
    }

    /// Creates an `Appointment` from a string slice
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentTime};
    ///
    /// let appointment = Appointment::from("15:06 Visit parents").unwrap();
    /// assert_eq!(Appointment::new(String::from("Visit parents"), AppointmentTime::new(15, 6).unwrap()), appointment);
    /// ```
    pub fn from(appointment: &str) -> Result<Self, String> {
        let time: String = appointment.chars().take(5).collect();
        let appointment_time = AppointmentTime::from(&time)?;
        let description = appointment.chars().skip(6).collect();
        Ok(Appointment::new(description, appointment_time))
    }

    /// Checks if the current time is earlier or equal to a reference to another AppointmentTime
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentTime};
    ///
    /// let appointment = Appointment::new(String::from("Appointment 1"), AppointmentTime::new(9, 56).unwrap());
    /// assert!(appointment.is_equal_or_earlier_than(&AppointmentTime::new(16, 3).unwrap()));
    /// ```
    pub fn is_equal_or_earlier_than(&self, appointment_time: &AppointmentTime) -> bool {
        self.time.is_equal_or_earlier_than(appointment_time)
    }

    /// Return a string version of the `Appointment` for displaying to the user. If the time of the
    /// current appointment is earlier or equal to the reference time informed, the string is
    /// returned with a `strikethrough`
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentTime};
    ///
    /// let appointment = Appointment::new(String::from("A complete description"), AppointmentTime::new(10, 29).unwrap());
    /// assert_eq!("[10:29] A complete description", appointment.to_string_display(&AppointmentTime::new(9, 24).unwrap()));
    /// ```
    pub fn to_string_display(&self, ref_time: &AppointmentTime) -> String {
        let display = format!("[{}] {}", self.time, self.description);
        if self.is_equal_or_earlier_than(ref_time) {
            display.strikethrough().to_string()
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
    use colored::Colorize;

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
    fn display_appointment_edge_case() {
        let appointment = Appointment::new(
            String::from("Study for test tomorrow"),
            AppointmentTime::new(12, 4).unwrap(),
        );
        let ref_time = AppointmentTime::new(12, 5).unwrap();
        assert_eq!(
            "[12:04] Study for test tomorrow"
                .strikethrough()
                .to_string(),
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn display_appointment_edge_case_complement() {
        let appointment = Appointment::new(
            String::from("Go to gym"),
            AppointmentTime::new(5, 30).unwrap(),
        );
        let ref_time = AppointmentTime::new(5, 30).unwrap();
        assert_eq!(
            "[05:30] Go to gym".strikethrough().to_string(),
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn display_past_appointment() {
        let appointment = Appointment::new(
            String::from("Do the laundry"),
            AppointmentTime::new(18, 0).unwrap(),
        );
        let ref_time = AppointmentTime::new(18, 5).unwrap();
        assert_eq!(
            "[18:00] Do the laundry".strikethrough().to_string(),
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn display_past_appointment_edge_case() {
        let appointment = Appointment::new(
            String::from("Make dinner"),
            AppointmentTime::new(20, 5).unwrap(),
        );
        let ref_time = AppointmentTime::new(20, 5).unwrap();
        assert_eq!(
            "[20:05] Make dinner".strikethrough().to_string(),
            appointment.to_string_display(&ref_time)
        );
    }

    #[test]
    fn appointment_should_be_passed() {
        let future_appointment = Appointment::new(
            String::from("Some future appointment"),
            AppointmentTime::now() + 5,
        );
        assert!(!future_appointment.is_equal_or_earlier_than(&AppointmentTime::now()))
    }

    #[test]
    fn appointment_not_should_be_passed() {
        let future_appointment = Appointment::new(
            String::from("Some past appointment"),
            AppointmentTime::now() - 5,
        );
        assert!(future_appointment.is_equal_or_earlier_than(&AppointmentTime::now()))
    }
}
