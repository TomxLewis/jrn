use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TimeStampFmt {
    Default,
}

impl TimeStampFmt {
    fn get_fmt_str(&self) -> &str {
        match self {
            _Default => "%Y-%m-%d_%H:%M",
        }
    }

    pub fn get_time_string(&self) -> String {
        let fmt_str: &str = self.get_fmt_str();
        let time: DateTime<Local> = Local::now();
        time.format(fmt_str).to_string()
    }
}

impl Default for TimeStampFmt {
    fn default() -> Self {
        TimeStampFmt::Default
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
