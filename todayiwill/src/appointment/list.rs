use core::fmt;
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

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
    pub fn new(reference_time: &AppointmentTime) -> Self {
        Self {
            appointments: vec![],
            reference_time: reference_time.clone(),
        }
    }

    pub fn from_path(reference_time: &AppointmentTime, path: &PathBuf) -> Self {
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

    #[allow(dead_code)]
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

    pub fn add(&mut self, appointment: Appointment, path: &PathBuf) -> Result<(), String> {
        self.appointments.push(appointment);
        self.appointments.sort();
        self.write(path)?;
        Ok(())
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), String> {
        match self.write_to_file(path) {
            Ok(..) => Ok(()),
            Err(error) => Err(format!(
                "Error while saving the appointment. Error: {}",
                error
            )),
        }
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

    #[allow(dead_code)]
    pub fn clear(&mut self, path: &PathBuf) -> Result<(), String> {
        self.appointments = vec![];
        match fs::remove_file(path) {
            Ok(..) => Ok(()),
            Err(error) => Err(format!(
                "An error occurred when clearing the appointments. {}",
                error
            )),
        }
    }

    fn write_to_file(&self, path: &PathBuf) -> Result<(), io::Error> {
        fs::create_dir_all(path.parent().unwrap())?;
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for appointment in &self.appointments {
            writeln!(writer, "{}", appointment)?;
        }
        Ok(())
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

    use crate::appointment::{
        list::{AppointmentList, ListOptions},
        Appointment, AppointmentTime,
    };

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

        let list = AppointmentList::from_path(&AppointmentTime::now(), &test_file_path);
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
        let list = AppointmentList::from_path(&AppointmentTime::now(), &test_file_path);
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
        let list = AppointmentList::from_appointments(AppointmentTime::now(), appointments);
        assert_eq!(
            "[14:30] Feed my pet fish\n[08:50] Clean my bedroom",
            list.to_string()
        );
    }

    #[test]
    fn filter_should_retain_by_time() {
        let appointments = vec![
            Appointment::new(
                String::from("Close the windows"),
                AppointmentTime::new(6, 20).unwrap(),
            ),
            Appointment::new(
                String::from("Feed the dog"),
                AppointmentTime::new(7, 25).unwrap(),
            ),
            Appointment::new(
                String::from("Check the news"),
                AppointmentTime::new(8, 0).unwrap(),
            ),
        ];
        let mut list = AppointmentList::from_appointments(
            AppointmentTime::new(7, 34).unwrap(),
            appointments.to_vec(),
        );
        assert_eq!(
            vec![Appointment::new(
                String::from("Check the news"),
                AppointmentTime::new(8, 0).unwrap(),
            ),],
            list.filter(ListOptions::ByReferenceTime).appointments()
        );
    }

    #[test]
    fn appointments_write_to_file() {
        let path = PathBuf::from("/tmp")
            .join("todayilearn-test-add")
            .join("test_file.txt");
        fs::create_dir_all(path.parent().unwrap().to_str().unwrap())
            .expect("Failed to create data dir");
        if path.exists() {
            fs::remove_file(&path).expect("Failed to clean test file");
        }
        assert!(!path.exists());
        let appointments = vec![
            Appointment::new(
                String::from("Call aunt Anna"),
                AppointmentTime::new(15, 46).unwrap(),
            ),
            Appointment::new(
                String::from("Buy new cup"),
                AppointmentTime::new(16, 56).unwrap(),
            ),
        ];
        let list = AppointmentList::from_appointments(AppointmentTime::now(), appointments);
        list.write(&path)
            .expect("Failed to write appointments on file");
        let file_result = fs::read_to_string(&path).expect("Failed to read file content");
        assert_eq!("15:46 Call aunt Anna\n16:56 Buy new cup\n", file_result);
    }

    #[test]
    fn add_appointments_should_be_ok() {
        let path = PathBuf::from("/tmp")
            .join("todayilearn-test-add-2")
            .join("test_file.txt");
        fs::create_dir_all(path.parent().unwrap().to_str().unwrap())
            .expect("Failed to create data dir");
        let mut list = AppointmentList::from_appointments(
            AppointmentTime::now(),
            vec![
                Appointment::new(
                    String::from("Go to the bank"),
                    AppointmentTime::new(15, 30).unwrap(),
                ),
                Appointment::new(
                    String::from("Wash the dishes"),
                    AppointmentTime::new(9, 15).unwrap(),
                ),
            ],
        );
        if path.exists() {
            fs::remove_file(&path).expect("Failed to clean test file");
        }
        assert!(!path.exists());
        list.add(
            Appointment::new(
                String::from("Visit cousin Frank"),
                AppointmentTime::new(10, 43).unwrap(),
            ),
            &path,
        )
        .expect("Failed to add a new appointment to the list");
        let file_content = fs::read_to_string(&path).expect("Failed to read file content");
        assert_eq!(
            "09:15 Wash the dishes\n10:43 Visit cousin Frank\n15:30 Go to the bank\n",
            file_content
        );
    }

    #[test]
    fn clear_appointments_should_be_ok() {
        let path = PathBuf::from("/tmp")
            .join("todayilearn-test-clear")
            .join("test_file.txt");
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create data dir");
        let mut file = File::create(path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(b"12:54 A random appointment\n")
            .expect("Failed to write to test file");
        assert!(path.exists());
        let mut list = AppointmentList::from_path(&AppointmentTime::now(), &path);
        list.clear(&path).expect("Failed to remove file");
        assert!(!path.exists());
    }
}
