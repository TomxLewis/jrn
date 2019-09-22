use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TimeStampFmt(String);

impl FromStr for TimeStampFmt {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TimeStampFmt(String::from(s)))
    }
}

impl TimeStampFmt {
    pub fn get_time_string(&self) -> String {
       let time: DateTime<Local> = Local::now();
        time.format(&self.0).to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq)]
pub struct UtcOffset {
    local_minus_utc: i32,
}

impl UtcOffset {
    pub fn local() -> UtcOffset {
        let local_time: DateTime<Local> = Local::now();
        let local_minus_utc = local_time.offset().local_minus_utc();
        UtcOffset {
            local_minus_utc
        }
    }
}

impl From<&FixedOffset> for UtcOffset {
    fn from(offset: &FixedOffset) -> Self {
        UtcOffset {
            local_minus_utc: offset.local_minus_utc(),
        }
    }
}
