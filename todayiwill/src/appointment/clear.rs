use std::{fs, io, path::PathBuf};

use super::Config;

pub fn clear_appointments(config: Config) {
    match remove_file(&config.appointments_path) {
        Ok(..) => println!("Appointments cleared successfully."),
        Err(error) => println!(
            "An error occurred when clearing the appointments. {}",
            error
        ),
    }
}

fn remove_file(path: &PathBuf) -> Result<(), io::Error> {
    fs::remove_file(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    use crate::appointment::clear::remove_file;

    #[test]
    fn remove_file_successfully() {
        let path = PathBuf::from("/tmp")
            .join("todayilearn-test-clear")
            .join("test_file.txt");
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create data dir");
        let mut file = File::create(path.to_str().unwrap()).expect("Failed to create test file");
        file.write_all(b"12:54 A random appointment\n")
            .expect("Failed to write to test file");
        assert!(path.exists());
        remove_file(&path).expect("Failed to remove file");
        assert!(!path.exists());
    }
}
