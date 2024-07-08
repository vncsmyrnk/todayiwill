use chrono::{Local, NaiveDate, ParseError};
use std::path::PathBuf;

use super::helper;

/// Represent the config needed to save the appointments
pub struct Config {
    pub appointment_file_path_current_day: Box<PathBuf>,
    pub appointment_file_path_builder: Box<dyn Fn(NaiveDate) -> PathBuf>,
}

impl Config {
    /// Returns the standard config with the usable paths according to the data dir of the current
    /// operating system
    ///
    /// # Example
    ///
    /// ```
    /// #[cfg(target_os = "linux")]
    /// use todayiwill::Config;
    /// use std::path::PathBuf;
    /// use chrono::{Local, NaiveDate};
    ///
    /// let config = Config::standard();
    /// assert_eq!(PathBuf::from(format!("/home/{}/.local/share/todayiwill/appointments_01012024.txt", whoami::username())), (config.appointment_file_path_builder)(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
    /// assert_eq!(PathBuf::from(format!("/home/{}/.local/share/todayiwill/appointments_{}.txt", whoami::username(), Local::now().format("%d%m%Y"))), *config.appointment_file_path_current_day);
    /// ```
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

/// Parses string time (hours and minutes) and returns a tuple with both values
///
/// # Example
///
/// ```
/// use todayiwill::helper;
///
/// assert_eq!((10, 43), helper::parse_time("10:43").unwrap());
/// ```
pub fn parse_time(time: &str) -> Option<(i32, i32)> {
    let (hour_str, minutes_str) = time.split_once(':')?;
    let hour = hour_str.parse().ok()?;
    let minutes = minutes_str.parse().ok()?;
    Some((hour, minutes))
}

/// Returns a string code for a given date
///
/// # Example
///
/// ```
/// use todayiwill::helper;
/// use chrono::NaiveDate;
///
/// assert_eq!("31012022", helper::date_code(NaiveDate::from_ymd_opt(2022, 1, 31).unwrap()));
/// ```
pub fn date_code(date: NaiveDate) -> String {
    date.format("%d%m%Y").to_string()
}

/// Converts a string to a naive date
///
/// # Example
///
/// ```
/// use todayiwill::helper;
/// use chrono::NaiveDate;
///
/// assert_eq!(NaiveDate::from_ymd_opt(2020, 11, 20).unwrap(), helper::str_dmy_to_naive_date("20/11/2020").unwrap());
/// ```
pub fn str_dmy_to_naive_date(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, "%d/%m/%Y")
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{date_code, parse_time, str_dmy_to_naive_date, Config};

    #[test]
    fn parse_wellformed_time() {
        let result = parse_time("22:45").unwrap();
        assert_eq!(result, (22, 45));
    }

    #[test]
    fn parse_malformed_time() {
        let result = parse_time("0x:21e");
        assert!(result.is_none());
    }

    #[test]
    fn date_code_check() {
        let result = date_code(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());
        assert_eq!(result, "02012024");
    }

    #[test]
    fn wellformed_date_naive_parse() {
        let result = str_dmy_to_naive_date("24/06/2023");
        assert_eq!(
            result.unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 24).unwrap()
        );
    }

    #[test]
    fn malformed_date_naive_parse() {
        let result = str_dmy_to_naive_date("12/96202");
        assert!(result.is_err());
    }

    #[test]
    fn malformed_date_naive_parse_edge_case() {
        let result = str_dmy_to_naive_date("2020-01-23");
        assert!(result.is_err());
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
