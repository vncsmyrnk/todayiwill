use chrono::{DateTime, Local};

/// Parses string time (hours and minutes) and returns a tuple with both values
/// `10:43` -> Option<(10, 43)>
pub fn parse_time(time: &str) -> Option<(i32, i32)> {
    let (hour_str, minutes_str) = time.split_once(':')?;
    let hour = hour_str.parse().ok()?;
    let minutes = minutes_str.parse().ok()?;
    Some((hour, minutes))
}

pub fn current_date_timestamp() -> Result<i64, chrono::format::ParseError> {
    let current_date = Local::now().format("%d/%m/%Y").to_string();
    let date_time = DateTime::parse_from_str("%d/%m/%Y %H:%M:%S", format!("{current_date} 00:00:00").as_str())?;
    Ok(date_time.timestamp())
}

#[cfg(test)]
mod tests {
    use super::{current_date_timestamp, parse_time};

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
    fn timestamp_check() {
        let current_timestamp = current_date_timestamp().unwrap();
        assert_eq!(current_timestamp, 1719457200);
    }
}
