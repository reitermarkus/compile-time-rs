use time::{Date, OffsetDateTime, Time};

const COMPILE_DATE: Date = compile_time::date!();
const COMPILE_TIME: Time = compile_time::time!();
const COMPILE_DATETIME: OffsetDateTime = compile_time::datetime!();

const COMPILE_DATE_STRING: &str = compile_time::date_str!();
const COMPILE_TIME_STRING: &str = compile_time::time_str!();
const COMPILE_DATETIME_STRING: &str = compile_time::datetime_str!();

#[cfg(test)]
mod tests {
  use time::Duration;

  use super::*;

  #[test]
  fn date_is_cached() {
    assert_eq!(COMPILE_DATE, compile_time::date!());
  }

  #[test]
  fn time_is_cached() {
    assert_eq!(COMPILE_TIME, compile_time::time!());
  }

  #[test]
  fn datetime_is_cached() {
    assert_eq!(COMPILE_DATETIME, compile_time::datetime!());
  }

  #[test]
  fn date_format() {
    let year = COMPILE_DATE.year();
    let month = u8::from(COMPILE_DATE.month());
    let day = COMPILE_DATE.day();
    let s = format!("{:04}-{:02}-{:02}", year, month, day);
    assert_eq!(COMPILE_DATE_STRING, s);
  }

  #[test]
  fn time_format() {
    let hour = COMPILE_TIME.hour();
    let minute = COMPILE_TIME.minute();
    let second = COMPILE_TIME.second();
    let s = format!("{:02}:{:02}:{:02}", hour, minute, second);
    assert_eq!(COMPILE_TIME_STRING, s);
  }

  #[test]
  fn datetime_format() {
    let year = COMPILE_DATETIME.year();
    let month = u8::from(COMPILE_DATETIME.month());
    let day = COMPILE_DATETIME.day();
    let hour = COMPILE_DATETIME.hour();
    let minute = COMPILE_DATETIME.minute();
    let second = COMPILE_DATETIME.second();
    let s = format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, minute, second);
    assert_eq!(COMPILE_DATETIME_STRING, s);
  }

  #[test]
  fn datetime_range() {
    let now = time::OffsetDateTime::now_utc();
    let yesterday = now.saturating_sub(Duration::days(1));
    assert!(COMPILE_DATETIME > yesterday);
    assert!(COMPILE_DATETIME < now);
  }
}
