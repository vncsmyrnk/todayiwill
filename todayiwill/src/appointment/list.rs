use core::fmt;
use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use crate::appointment::AppointmentTime;

use super::Appointment;

/// Describe the filter options available for filtering appointments
pub enum FilterOption {
    ByReferenceTime,
    ByReferenceAndExpireTime(i32),
}

/// Describe a list of appointments
pub struct AppointmentList<'a> {
    reference_time: &'a AppointmentTime,
    path: &'a PathBuf,
    appointments: Vec<Appointment>,
}

impl<'a> AppointmentList<'a> {
    /// Initialize a list. The appointments will be loaded from the path informed. Subsequent
    /// operations will consider the same path
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::path::PathBuf;
    ///
    /// let reference_time = AppointmentTime::new(12, 45).unwrap();
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments.txt");
    /// let list = AppointmentList::new(&reference_time, &path);
    /// assert_eq!(&Vec::<Appointment>::new(), list.appointments());
    /// ```
    pub fn new(reference_time: &'a AppointmentTime, path: &'a PathBuf) -> Self {
        let mut new_appointment = Self {
            reference_time,
            path,
            appointments: vec![],
        };
        new_appointment.load();
        new_appointment
    }

    /// Returns a reference of the current state of appontments
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::path::PathBuf;
    ///
    /// let reference_time = AppointmentTime::new(1, 50).unwrap();
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments.txt");
    /// let list = AppointmentList::new(&reference_time, &path);
    /// assert_eq!(&Vec::<Appointment>::new(), list.appointments());
    /// ```
    pub fn appointments(&self) -> &Vec<Appointment> {
        &self.appointments
    }

    /// Returns if the vector of appointments is currently empty
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::path::PathBuf;
    ///
    /// let reference_time = AppointmentTime::new(18, 24).unwrap();
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments.txt");
    /// let list = AppointmentList::new(&reference_time, &path);
    /// assert!(list.no_appointments());
    /// ```
    pub fn no_appointments(&self) -> bool {
        self.appointments.is_empty()
    }

    /// Reads the current path and fill the appointments vector. It is automatically done at
    /// AppointmentList instantiation
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, fs::File, io::Write, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_load.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// let mut file = File::create(path.to_str().unwrap()).expect("Failed to create test file");
    /// file.write_all(b"07:45 Example appointment").expect("Failed to write to test file");
    ///
    /// let reference_time = AppointmentTime::now();
    /// let list = AppointmentList::new(&reference_time, &path);
    /// assert_eq!(&vec![Appointment::new(String::from("Example appointment"), AppointmentTime::new(7, 45).unwrap())], list.appointments());
    /// ```
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

    /// Add an appointment to the list and the file
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_add.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// if path.exists() {
    ///     fs::remove_file(&path).expect("Failed to clean test file");
    /// }
    ///
    /// let reference_time = AppointmentTime::now();
    /// let mut list = AppointmentList::new(&reference_time, &path);
    /// list.add(Appointment::new(String::from("New appointment"), AppointmentTime::new(14, 8).unwrap())).unwrap();
    /// assert_eq!(&vec![Appointment::new(String::from("New appointment"), AppointmentTime::new(14, 8).unwrap())], list.appointments());
    /// assert_eq!("14:08 New appointment\n", fs::read_to_string(&path).expect("Failed to read file content"));
    /// ```
    pub fn add(&mut self, appointment: Appointment) -> Result<(), String> {
        self.appointments.retain(|a| a.time != appointment.time);
        self.appointments.push(appointment);
        self.appointments.sort();
        self.write()?;
        Ok(())
    }

    /// Removes an appointment from the list and the file
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_remove.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// if path.exists() {
    ///     fs::remove_file(&path).expect("Failed to clean test file");
    /// }
    ///
    /// let reference_time = AppointmentTime::now();
    /// let mut list = AppointmentList::new(&reference_time, &path);
    /// list.add(Appointment::new(String::from("New appointment"), AppointmentTime::new(5, 32).unwrap()));
    /// list.add(Appointment::new(String::from("Other appointment"), AppointmentTime::new(22, 48).unwrap()));
    /// list.remove(AppointmentTime::new(22, 48).unwrap()).unwrap();
    /// assert_eq!(&vec![Appointment::new(String::from("New appointment"), AppointmentTime::new(5, 32).unwrap())], list.appointments());
    /// assert_eq!("05:32 New appointment\n", fs::read_to_string(&path).expect("Failed to read file content"));
    /// ```
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

    /// Write the current state of appointments to the path. It is automaically done in some
    /// operations
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_write.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// if path.exists() {
    ///     fs::remove_file(&path).expect("Failed to clean test file");
    /// }
    ///
    /// let reference_time = AppointmentTime::now();
    /// let mut list = AppointmentList::new(&reference_time, &path);
    /// list.add(Appointment::new(String::from("New appointment"), AppointmentTime::new(5, 32).unwrap()));
    /// list.add(Appointment::new(String::from("Other appointment"), AppointmentTime::new(22, 48).unwrap()));
    /// list.write().unwrap();
    /// assert_eq!("05:32 New appointment\n22:48 Other appointment\n", fs::read_to_string(&path).expect("Failed to read file content"));
    /// ```
    pub fn write(&self) -> Result<(), String> {
        match self.write_to_file() {
            Ok(..) => Ok(()),
            Err(error) => Err(format!(
                "Error while saving the appointment. Error: {}",
                error
            )),
        }
    }

    /// Applies a filter type and overrides the appointment vector. The filter possibilities are
    /// defined in `FilterOption`
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime, FilterOption};
    /// use std::{fs, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_filter.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// if path.exists() {
    ///     fs::remove_file(&path).expect("Failed to clean test file");
    /// }
    ///
    /// let reference_time = AppointmentTime::new(18, 54).unwrap();
    /// let mut list = AppointmentList::new(&reference_time, &path);
    /// list.add(Appointment::new(String::from("New appointment"), AppointmentTime::new(18, 56).unwrap()));
    /// list.add(Appointment::new(String::from("Other appointment"), AppointmentTime::new(10, 2).unwrap()));
    /// list.filter(FilterOption::ByReferenceAndExpireTime(5));
    /// assert_eq!(&vec![Appointment::new(String::from("New appointment"), AppointmentTime::new(18, 56).unwrap())], list.appointments());
    /// ```
    pub fn filter(&mut self, options: FilterOption) -> &Self {
        let filter_by_reference_time = |a: &Appointment| a.time > *self.reference_time;
        match options {
            FilterOption::ByReferenceTime => {
                self.appointments.retain(filter_by_reference_time);
            }
            FilterOption::ByReferenceAndExpireTime(expire_in_seconds) => {
                self.appointments.retain(filter_by_reference_time);
                self.appointments
                    .retain(|a| a.time <= self.reference_time.clone() + expire_in_seconds);
            }
        }
        self
    }

    /// Copies the appointments created in another day to the current day
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, fs::File, io::Write, path::PathBuf};
    ///
    /// let path_origin = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_copy_origin.txt");
    /// fs::create_dir_all(path_origin.parent().unwrap()).expect("Failed to create test dir");
    /// if path_origin.exists() {
    ///     fs::remove_file(&path_origin).expect("Failed to clean test file");
    /// }
    /// let mut file = File::create(path_origin.to_str().unwrap()).expect("Failed to create test file");
    /// file.write_all(b"15:58 Example appointment").expect("Failed to write to test file");
    ///
    /// let path_dest = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_copy_destination.txt");
    /// if path_dest.exists() {
    ///     fs::remove_file(&path_dest).expect("Failed to clean test file");
    /// }
    ///
    /// let reference_time = AppointmentTime::now();
    /// let mut list = AppointmentList::new(&reference_time, &path_dest);
    /// assert!(list.no_appointments());
    /// list.copy(&path_origin).unwrap();
    /// assert_eq!(&vec![Appointment::new(String::from("Example appointment"), AppointmentTime::new(15, 58).unwrap())], list.appointments());
    /// ```
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

    /// Clears the appointments added for the current day
    ///
    /// # Example
    ///
    /// ```
    /// use todayiwill::{Appointment, AppointmentList, AppointmentTime};
    /// use std::{fs, fs::File, io::Write, path::PathBuf};
    ///
    /// let path = PathBuf::from("/tmp").join("todayiwill").join("appointments_test_clear.txt");
    /// fs::create_dir_all(path.parent().unwrap()).expect("Failed to create test dir");
    /// let mut file = File::create(path.to_str().unwrap()).expect("Failed to create test file");
    /// file.write_all(b"22:15 Example appointment").expect("Failed to write to test file");
    ///
    /// let reference_time = AppointmentTime::now();
    /// let mut list = AppointmentList::new(&reference_time, &path);
    /// assert_eq!(&vec![Appointment::new(String::from("Example appointment"), AppointmentTime::new(22, 15).unwrap())], list.appointments());
    /// list.clear().unwrap();
    /// assert!(list.no_appointments());
    /// ```
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

    /// Writes the appointments vector in the path supplied
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
        list::{AppointmentList, FilterOption},
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
    fn parse_and_load_file_content_should_be_ok() {
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
    fn non_existent_file_should_be_parsed_as_empty_vec() {
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
    fn filter_should_retain_by_reference_time() {
        let path = generate_path_for_test("filter_should_retain_by_reference_time");
        let reference_time = AppointmentTime::new(7, 29).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Close the windows"),
            AppointmentTime::new(7, 29).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Feed the dog"),
            AppointmentTime::new(7, 25).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Check the news"),
            AppointmentTime::new(7, 30).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            &vec![Appointment::new(
                String::from("Check the news"),
                AppointmentTime::new(7, 30).unwrap(),
            ),],
            list.filter(FilterOption::ByReferenceTime).appointments()
        );
    }

    #[test]
    fn filter_should_retain_by_reference_and_expire_time() {
        let path = generate_path_for_test("filter_should_retain_by_reference_and_expire_time");
        let reference_time = AppointmentTime::new(16, 23).unwrap();
        let mut list = AppointmentList::new(&reference_time, &path);
        list.add(Appointment::new(
            String::from("Format pc"),
            AppointmentTime::new(18, 1).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Do the laundry"),
            AppointmentTime::new(16, 28).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("Check the news"),
            AppointmentTime::new(16, 29).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            &vec![Appointment::new(
                String::from("Do the laundry"),
                AppointmentTime::new(16, 28).unwrap(),
            ),],
            list.filter(FilterOption::ByReferenceAndExpireTime(5))
                .appointments()
        );
    }

    #[test]
    fn writing_appointments_to_file_should_be_ok() {
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

        list.add(Appointment::new(
            String::from("Clean kitchen floor"),
            AppointmentTime::new(20, 45).unwrap(),
        ))
        .unwrap();

        list.write().expect("Failed to write appointments on file");

        assert_eq!(
            "15:46 Call aunt Anna\n16:56 Buy new cup\n20:45 Clean kitchen floor\n",
            fs::read_to_string(&path).expect("Failed to read file content")
        );

        list.remove(AppointmentTime::new(16, 56).unwrap()).unwrap();

        assert_eq!(
            "15:46 Call aunt Anna\n20:45 Clean kitchen floor\n",
            fs::read_to_string(&path).expect("Failed to read file content")
        );

        list.add(Appointment::new(
            String::from("Appointment to override existent"),
            AppointmentTime::new(15, 46).unwrap(),
        ))
        .unwrap();

        assert_eq!(
            "15:46 Appointment to override existent\n20:45 Clean kitchen floor\n",
            fs::read_to_string(&path).expect("Failed to read file content")
        );
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

        assert_eq!(
            &vec![
                Appointment::new(
                    String::from("Wash the dishes"),
                    AppointmentTime::new(9, 15).unwrap()
                ),
                Appointment::new(
                    String::from("Visit cousin Frank"),
                    AppointmentTime::new(10, 43).unwrap()
                ),
                Appointment::new(
                    String::from("Go to the bank"),
                    AppointmentTime::new(15, 30).unwrap()
                )
            ],
            list.appointments()
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

    #[test]
    fn copy_should_be_ok() {
        let path_origin = generate_path_for_test("copy_should_be_ok_origin");
        write_to_path(&path_origin, b"09:23 The original appointment");

        let reference_time = AppointmentTime::new(12, 46).unwrap();
        let path_dist = generate_path_for_test("copy_should_be_ok_dist");
        let mut list = AppointmentList::new(&reference_time, &path_dist);
        assert!(list.no_appointments());

        list.copy(&path_origin).unwrap();
        assert_eq!(
            &vec![Appointment::new(
                String::from("The original appointment"),
                AppointmentTime::new(9, 23).unwrap()
            )],
            list.appointments()
        );
    }

    #[test]
    fn remove_should_be_ok() {
        let reference_time = AppointmentTime::new(22, 7).unwrap();
        let path = generate_path_for_test("remove_should_be_ok");
        let mut list = AppointmentList::new(&reference_time, &path);

        list.add(Appointment::new(
            String::from("Appointment to be removed"),
            AppointmentTime::new(23, 2).unwrap(),
        ))
        .unwrap();
        list.add(Appointment::new(
            String::from("This appointment should not be removed"),
            AppointmentTime::new(22, 55).unwrap(),
        ))
        .unwrap();

        list.remove(AppointmentTime::new(23, 2).unwrap())
            .expect("Failed to remove appointment by time");

        assert_eq!(
            &vec![Appointment::new(
                String::from("This appointment should not be removed"),
                AppointmentTime::new(22, 55).unwrap()
            )],
            list.appointments()
        );
    }
}
