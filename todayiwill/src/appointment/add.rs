use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use super::{list, Appointment, Config};

/// Adds a new appointment to the list stored in files
pub fn add_appointment(appointment: Appointment, config: Config) {
    let mut appointments = list::get_appointments_from_file(&config.appointments_path);
    appointments.push(appointment);
    match write_appointments_to_file(appointments, &config.appointments_path) {
        Ok(..) => println!("Appointment added successfully."),
        Err(error) => println!("An error occurred. {}", error),
    }
}

/// Append a new appointment on the file storage
fn write_appointments_to_file(
    appointments: Vec<Appointment>,
    path: &PathBuf,
) -> Result<(), io::Error> {
    fs::create_dir_all(path.parent().unwrap())?;
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for appointment in appointments {
        writeln!(writer, "{}", appointment)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::appointment::{Appointment, AppointmentTime};

    use super::write_appointments_to_file;

    #[test]
    fn appointments_write_to_file() {
        let base_path = PathBuf::from("/tmp").join("todayilearn-test-add");
        fs::create_dir_all(base_path.to_str().unwrap()).expect("Failed to create data dir");
        let path = base_path.join("test_file.txt");
        write_appointments_to_file(
            vec![
                Appointment::new(String::from("Call aunt Anna"), AppointmentTime::new(15, 46).unwrap()),
                Appointment::new(String::from("Buy new cup"), AppointmentTime::new(16, 56).unwrap()),
            ],
            &path,
        )
        .expect("Failed to write appointments on file");
        let file_result = fs::read_to_string(&path).expect("Failed to read file content");
        assert_eq!("15:46 Call aunt Anna\n16:56 Buy new cup\n", file_result);
    }
}
