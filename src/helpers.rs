use std::time::SystemTime;
use chrono::{DateTime, NaiveDateTime, Local};
use std::cmp::Ordering;
use crate::args::OrderDirection;

pub trait ToPrettySize {
    fn to_pretty_size(&self) -> String;
}

impl ToPrettySize for u64 {
    fn to_pretty_size(&self) -> String {
        let units = ["bytes", "KB", "MB", "GB", "TB"];
        let mut size = *self as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < units.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        let formatted_size = format!("{:.3}", size);
        let trimmed_size = formatted_size.trim_end_matches('0').trim_end_matches('.');

        format!("{} {}", trimmed_size, units[unit_index])
    }
}

pub const DATE_FORMAT: &str = "%d/%m/%Y %T";

pub trait SystemTimeStringHelpers {
    fn to_string(&self) -> String;
    fn compare(a: &String, b: &String, direction: &OrderDirection) -> Ordering;
}

impl SystemTimeStringHelpers for SystemTime {
    fn to_string(&self) -> String {
        let date_time: DateTime<Local> = (*self).into();
        date_time.format(DATE_FORMAT).to_string()
    }

    fn compare(a: &String, b: &String, direction: &OrderDirection) -> Ordering {
        let date_a = NaiveDateTime::parse_from_str(a.as_str(), DATE_FORMAT).unwrap();
        let date_b = NaiveDateTime::parse_from_str(b.as_str(), DATE_FORMAT).unwrap();

        match direction {
            OrderDirection::Ascending => date_a.cmp(&date_b),
            OrderDirection::Descending => date_b.cmp(&date_a),
        }
    }
}
