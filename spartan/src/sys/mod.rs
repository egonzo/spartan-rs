use std::time::{Duration, SystemTime};

pub mod gdrive;
pub mod mgo;
pub mod slack;
pub mod sync;

/// Subtracts number of days from date. If there is an error date is returned.
pub fn sub_date(date: SystemTime, days: u64) -> SystemTime {
    let duration = Duration::from_secs(days * (60 * 60 * 24));

    date.checked_sub(duration).unwrap_or_else(|| date)
}
