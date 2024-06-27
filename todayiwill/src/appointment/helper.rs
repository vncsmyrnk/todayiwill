use chrono::{Local, NaiveDate};

/// Parses string time (hours and minutes) and returns a tuple with both values
/// `10:43` -> Option<(10, 43)>
pub fn parse_time(time: &str) -> Option<(i32, i32)> {
    let (hour_str, minutes_str) = time.split_once(':')?;
    let hour = hour_str.parse().ok()?;
    let minutes = minutes_str.parse().ok()?;
    Some((hour, minutes))
}

/// Returns a string code for a given date
/// Option<20/01/2021> -> "20012021"
pub fn date_code(date: NaiveDate) -> String {
    date.format("%d%m%Y").to_string()
}

/// Returns a date code for the current date
pub fn current_date_code() -> String {
    date_code(Local::now().date_naive())
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate};

    use crate::appointment::helper::current_date_code;

    use super::{date_code, parse_time};

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
    fn current_date_code_check() {
        let result = current_date_code();
        assert_eq!(result, Local::now().format("%d%m%Y").to_string());
    }
}
