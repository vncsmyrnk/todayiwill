/// Parses string time (hours and minutes) and returns a tuple with both values
/// `10:43` -> Option<(10, 43)>
pub fn parse_time(time: &str) -> Option<(i32, i32)> {
    let (hour_str, minutes_str) = time.split_once(':')?;
    let hour = hour_str.parse().ok()?;
    let minutes = minutes_str.parse().ok()?;
    Some((hour, minutes))
}

#[cfg(test)]
mod tests {
    use super::parse_time;

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
}
