use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use std::str::FromStr;

const SECONDS_IN_HOUR: i32 = 3600;
const UTC_TO_CHINESE_HOURS: i32 = 8;
const UTC_TO_CHINESE_SECONDS: i32 = UTC_TO_CHINESE_HOURS * SECONDS_IN_HOUR;

fn insight() {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    println!("Seconds from 1970 to today: {seconds}");

    let utc_dt = DateTime::from_timestamp(seconds as i64, 0).unwrap();
    println!("As DateTime<Utc>: {utc_dt}");

    let naive_dt = NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    println!("As NaiveDateTime: {naive_dt}");

    let utc_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);
    println!("As DateTime<Utc>: {utc_dt}");

    // 东八区
    let east_offset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    let east_dt: DateTime::<FixedOffset> = DateTime::from_utc(naive_dt, east_offset);
    println!("In a timezone 8 hours from UTC: {east_dt}");

    let east_naive_dt = east_dt.naive_local();
    println!("With timezone information removed: {east_naive_dt}");
}

#[derive(Debug)]
struct UtcUserEvent {
    timestamp: &'static str,
    data: String,
}

#[derive(Debug)]
struct ChinaUserEvent {
    timestamp: DateTime<FixedOffset>,
    data: String
}

impl From<UtcUserEvent> for ChinaUserEvent {
    fn from(e: UtcUserEvent) -> Self {
        let utc_dt: DateTime<Utc> = DateTime::from_str(e.timestamp).unwrap();
        let offset = FixedOffset::east_opt(UTC_TO_CHINESE_SECONDS).unwrap();
        let timestamp: DateTime<FixedOffset> = DateTime::from_utc(utc_dt.naive_utc(), offset);

        ChinaUserEvent {
            timestamp,
            data: e.data
        }
    }
}

fn main() {
    let incoming_event = UtcUserEvent {
        timestamp: "2024-03-13 13:35:27 UTC",
        data: "Something happend in UTC time".to_string(),
    };

    let china_event = ChinaUserEvent::from(incoming_event);
    println!("China user event: {china_event:?}");
}

#[test]
fn utc_to_china_output_same_evening() {
    let morning_event = UtcUserEvent {
        timestamp: "2024-03-13 09:48:50 UTC",
        data: String::new(),
    };

    let to_china = ChinaUserEvent::from(morning_event);
    assert_eq!(
        &to_china.timestamp.to_string(),
        "2024-03-13 17:48:50 +08:00"
    );
}

#[test]
fn utc_to_china_output_next_morning() {
    let evening_event = UtcUserEvent {
        timestamp: "2023-03-27 23:59:59 UTC",
        data: String::new(),
    };

    let to_china = ChinaUserEvent::from(evening_event);
    assert_eq!(
        &to_china.timestamp.to_string(),
        "2023-03-28 07:59:59 +08:00"
    );
}