use core::fmt;
use std::{fs, path::PathBuf};

use crate::appointment::AppointmentTime;

use super::Appointment;

pub enum ListOptions {
    ByReferenceTime,
    ByReferenceAndExpireTime(i32),
}

pub struct AppointmentList {
    reference_time: AppointmentTime,
    appointments: Vec<Appointment>,
}

impl AppointmentList {
    pub fn new(reference_time: AppointmentTime) -> Self {
        Self {
            appointments: vec![],
            reference_time,
        }
    }

    pub fn from_path(reference_time: AppointmentTime, path: &PathBuf) -> Self {
        let mut new_list = Self::new(reference_time);
        new_list.load(path);
        new_list
    }

    #[allow(dead_code)]
    pub fn from_appointments(
        reference_time: AppointmentTime,
        appointments: Vec<Appointment>,
    ) -> Self {
        Self {
            appointments,
            reference_time,
        }
    }

    pub fn appointments(&self) -> Vec<Appointment> {
        self.appointments.clone()
    }

    pub fn no_appointments(&self) -> bool {
        self.appointments.is_empty()
    }

    pub fn load(&mut self, path: &PathBuf) -> &Self {
        let file_result = fs::read_to_string(path);
        let file_content = match file_result {
            Ok(content) => content,
            Err(..) => String::new(),
        };
        let appointments: Vec<Result<Appointment, String>> =
            file_content.lines().map(Appointment::from).collect();
        self.appointments = appointments.into_iter().flatten().collect();
        self.appointments.sort();
        self
    }

    pub fn filter(&mut self, options: ListOptions) -> &Self {
        let filter_by_reference_time = |a: &Appointment| a.time > self.reference_time;
        match options {
            ListOptions::ByReferenceTime => {
                self.appointments.retain(filter_by_reference_time);
            }
            ListOptions::ByReferenceAndExpireTime(expire_in_seconds) => {
                self.appointments.retain(filter_by_reference_time);
                self.appointments
                    .retain(|a| a.time <= self.reference_time.clone() + expire_in_seconds);
            }
        }
        self
    }
}

impl fmt::Display for AppointmentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let appointments_text = self
            .appointments
            .iter()
            .map(|a| a.to_string_display(&self.reference_time))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{appointments_text}")
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, fs::File, io::Write, path::PathBuf};

    use crate::appointment::{list::AppointmentList, Appointment, AppointmentTime};

    #[test]
    fn parse_file_contents() {
        let test_file_path = PathBuf::from("/tmp")
            .join("todayilearn-test-list")
            .join("appointments.txt");
        fs::create_dir_all(test_file_path.parent().unwrap()).expect("Failed to create test dir");
        let mut file =
            File::create(test_file_path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(b"22:00 Go to night shift\n12:45 Visit grandma\n212 Nonsense")
            .expect("Failed to write to test file");

        let list = AppointmentList::from_path(AppointmentTime::now(), &test_file_path);
        let result = list.appointments();
        assert_eq!(
            result,
            vec![
                Appointment::new(
                    "Visit grandma".to_string(),
                    AppointmentTime::new(12, 45).unwrap()
                ),
                Appointment::new(
                    "Go to night shift".to_string(),
                    AppointmentTime::new(22, 0).unwrap()
                ),
            ]
        );
        fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn parse_file_non_existent() {
        let test_file_path = PathBuf::from("/tmp/non_existent.txt");
        let list = AppointmentList::from_path(AppointmentTime::now(), &test_file_path);
        let result = list.appointments();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn appointments_should_be_displayed_properly() {
        let appointments = vec![
            Appointment::new(
                String::from("Feed my pet fish"),
                AppointmentTime::new(14, 30).unwrap(),
            ),
            Appointment::new(
                String::from("Clean my bedroom"),
                AppointmentTime::new(8, 50).unwrap(),
            ),
        ];
        let display = AppointmentList::from_appointments(AppointmentTime::now(), appointments);
        assert_eq!(
            "[14:30] Feed my pet fish\n[08:50] Clean my bedroom",
            display.to_string()
        );
    }
}
