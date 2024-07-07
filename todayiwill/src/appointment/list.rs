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

pub struct AppointmentList<'a> {
    reference_time: &'a AppointmentTime,
    path: &'a PathBuf,
    appointments: Vec<Appointment>,
}

impl<'a> AppointmentList<'a> {
    pub fn new(reference_time: &'a AppointmentTime, path: &'a PathBuf) -> Self {
        let mut new_appointment = Self {
            reference_time,
            path,
            appointments: vec![],
        };
        new_appointment.load();
        new_appointment
    }

    pub fn appointments(&self) -> &Vec<Appointment> {
        &self.appointments
    }

    pub fn no_appointments(&self) -> bool {
        self.appointments.is_empty()
    }

    pub fn load(&mut self) -> &Self {
        let file_result = fs::read_to_string(self.path);
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

    pub fn add(&mut self, appointment: Appointment) -> Result<(), String> {
        self.appointments.retain(|a| a.time != appointment.time);
        self.appointments.push(appointment);
        self.appointments.sort();
        self.write()?;
        Ok(())
    }

    pub fn remove(&mut self, time: AppointmentTime) -> Result<(), String> {
        match self.appointments.iter().position(|a| a.time == time) {
            Some(index) => {
                if self.appointments[index].is_equal_or_past_from(self.reference_time) {
                    return Err(String::from(
                        "This appointment is already past and cannot be removed.",
                    ));
                }
                self.appointments.remove(index);
            }
            None => {
                return Err(String::from(
                    "There is no appointment at this specific time.",
                ))
            }
        };
        self.write()?;
        Ok(())
    }

    pub fn write(&self) -> Result<(), String> {
        match self.write_to_file() {
            Ok(..) => Ok(()),
            Err(error) => Err(format!(
                "Error while saving the appointment. Error: {}",
                error
            )),
        }
    }

    pub fn filter(&mut self, options: ListOptions) -> &Self {
        let filter_by_reference_time = |a: &Appointment| a.time > *self.reference_time;
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

    pub fn copy(&mut self, from: &PathBuf) -> Result<(), String> {
        if !self.appointments.is_empty() {
            return Err(String::from(
                "Copy not possible, there are appointments for the current day.",
            ));
        }
        if !from.exists() {
            return Err(String::from("Given day has no appointments."));
        }
        match fs::copy(from, self.path) {
            Ok(..) => {
                self.load();
                Ok(())
            }
            Err(error) => Err(format!(
                "An error ocurred while copying an appointments file. {}",
                error
            )),
        }
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.appointments = vec![];
        match fs::remove_file(self.path) {
            Ok(..) => Ok(()),
            Err(error) => Err(format!(
                "An error occurred while clearing the appointments. {}",
                error
            )),
        }
    }

    fn write_to_file(&self) -> Result<(), io::Error> {
        fs::create_dir_all(self.path.parent().unwrap())?;
        let file = File::create(self.path)?;
        let mut writer = BufWriter::new(file);
        for appointment in &self.appointments {
            writeln!(writer, "{}", appointment)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for AppointmentList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let appointments_text = self
            .appointments
            .iter()
            .map(|a| a.to_string_display(self.reference_time))
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

    use colored::Colorize;

    fn generate_path_for_test(test_name: &str) -> PathBuf {
        let path = PathBuf::from("/tmp")
            .join("todayilearn-tests")
            .join(format!("appointments_test_{}.txt", test_name));
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
        if path.exists() {
            fs::remove_file(&path).expect("Failed to clean test file");
        }
        path
    }

    fn write_to_path(path: &PathBuf, content: &[u8]) {
        let mut file = File::create(path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(content)
            .expect("Failed to write to test file");
    }

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

        let reference_time = AppointmentTime::new(11, 58).unwrap();
        let list = AppointmentList::new(&reference_time, &test_file_path);
        let result = list.appointments();
        assert_eq!(
            result,
            &vec![
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
        let reference_time = AppointmentTime::now();
        let list = AppointmentList::new(&reference_time, &test_file_path);
        let result = list.appointments();
        assert_eq!(result, &vec![]);
    }

    #[test]
    fn appointments_should_be_displayed_properly() {
        let path = generate_path_for_test("appointments_should_be_displayed_properly");
        let reference_time = AppointmentTime::new(5, 54).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Feed my pet fish"),
            AppointmentTime::new(14, 30).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Clean my bedroom"),
            AppointmentTime::new(8, 50).unwrap(),
        ))
        .unwrap();
        assert_eq!(
            "[08:50] Clean my bedroom\n[14:30] Feed my pet fish",
            list.to_string()
        );
    }

    #[test]
    fn filter_should_retain_by_time() {
        let path = generate_path_for_test("filter_should_retain_by_time");
        let reference_time = AppointmentTime::new(7, 29).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Close the windows"),
            AppointmentTime::new(6, 20).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Feed the dog"),
            AppointmentTime::new(7, 25).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Check the news"),
            AppointmentTime::new(8, 0).unwrap(),
        ))
        .unwrap();
        let reference_time = AppointmentTime::new(7, 34).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        assert_eq!(
            &vec![Appointment::new(
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

        let reference_time = AppointmentTime::new(3, 10).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Call aunt Anna"),
            AppointmentTime::new(15, 46).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Buy new cup"),
            AppointmentTime::new(16, 56).unwrap(),
        ))
        .unwrap();
        list.write().expect("Failed to write appointments on file");
        let file_result = fs::read_to_string(&path).expect("Failed to read file content");
        assert_eq!("15:46 Call aunt Anna\n16:56 Buy new cup\n", file_result);
    }

    #[test]
    fn add_appointments_should_be_ok() {
        let path = generate_path_for_test("add_appointments_should_be_ok");
        write_to_path(&path, b"15:30 Go to the bank\n09:15 Wash the dishes\n");

        let reference_time = AppointmentTime::new(6, 43).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Visit cousin Frank"),
            AppointmentTime::new(10, 43).unwrap(),
        ))
        .expect("Failed to add a new appointment to the list");
        let file_content = fs::read_to_string(&path).expect("Failed to read file content");
        assert_eq!(
            "09:15 Wash the dishes\n10:43 Visit cousin Frank\n15:30 Go to the bank\n",
            file_content
        );
    }

    #[test]
    fn clear_appointments_should_be_ok() {
        let path = generate_path_for_test("clear_appointments_should_be_ok");
        write_to_path(&path, b"12:54 A random appointment\n");
        assert!(path.exists());
        let reference_time = AppointmentTime::new(22, 4).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.clear().expect("Failed to remove file");
        assert!(!path.exists());
    }

    #[test]
    fn past_appointments_should_be_strikethrough() {
        let reference_time = AppointmentTime::new(14, 38).unwrap();
        let path = generate_path_for_test("past_appointments_should_be_strikethrough");
        let mut list = AppointmentList::new(&reference_time, &path);

        list.add(Appointment::new(
            String::from("Make restaurant reservations"),
            AppointmentTime::new(9, 47).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Update my professional portfolio"),
            AppointmentTime::new(19, 17).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Backup the vacation pictures"),
            AppointmentTime::new(12, 51).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Buy new sunglasses"),
            AppointmentTime::new(16, 8).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Play fifa"),
            AppointmentTime::new(14, 38).unwrap(),
        ))
        .unwrap();

        list.add(Appointment::new(
            String::from("Rest"),
            AppointmentTime::new(14, 39).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}",
                "[09:47] Make restaurant reservations"
                    .strikethrough()
                    .to_string(),
                "[12:51] Backup the vacation pictures"
                    .strikethrough()
                    .to_string(),
                "[14:38] Play fifa".strikethrough().to_string(),
                "[14:39] Rest",
                "[16:08] Buy new sunglasses",
                "[19:17] Update my professional portfolio"
            ),
            list.to_string()
        );
    }
}
