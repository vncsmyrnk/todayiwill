use core::fmt;

pub mod list;

pub struct AppointmentTime {
    pub hour: i32,
    pub minutes: i32,
}

impl AppointmentTime {
    pub fn new(hour: i32, minutes: i32) -> Self {
        Self { hour, minutes }
    }
}

impl fmt::Display for AppointmentTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
    }
}

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
        write!(f, "{} {}", self.description, self.time)
    }
}
