use std::str::FromStr;

use chrono::{DateTime, ParseError, Utc};

pub mod todo;

pub struct DateTimeUtils;

impl DateTimeUtils {
    pub fn to_string(date: DateTime<Utc>) -> String {
        date.format("%FT%TZ").to_string()
    }

    pub fn from_string(date: String) -> Result<DateTime<Utc>, ParseError> {
        DateTime::from_str(date.as_str())
    }
}