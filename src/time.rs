use chrono::prelude::*;
use serde::export::Formatter;
use serde::export::fmt::Error;

static TIMESTAMP_FMT: &str = "%Y-%m-%d_%H%M";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TimeStamp {
    inner: NaiveDateTime,
}

impl TimeStamp {
    pub fn now() -> Self {
        let dt: DateTime<Local> = Local::now();
        let ndt: NaiveDateTime = dt.naive_local();
        TimeStamp {
            inner: ndt,
        }
    }

    pub fn from_ymdhm(year: i32, month: u32, day: u32, hour: u32, minute:u32) -> Self {
        let date = NaiveDate::from_ymd(year, month, day);
        let ndt = date.and_hms(hour, minute, 0);
        TimeStamp {
            inner: ndt
        }
    }
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.inner.format(TIMESTAMP_FMT))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn can_write_timestamp() {
        let timestamp = TimeStamp::now().to_string();
        let path = Path::new(&timestamp);
        File::create(&path).unwrap();
        assert!(&path.exists());
        std::fs::remove_file(&path).unwrap();
    }
}