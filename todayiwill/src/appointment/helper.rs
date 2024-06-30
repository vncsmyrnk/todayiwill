use chrono::{NaiveDate, ParseError};

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

/// Converts a string to a naive date
pub fn str_dmy_to_naive_date(date: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date, "%d/%m/%Y")
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{date_code, parse_time, str_dmy_to_naive_date};

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
}
