use chrono::prelude::*;
use std::path::Path;

static TIMESTAMP_FMT: &'static str = "%Y-%m-%d_%H:%M";

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

    pub fn to_string(&self) -> String {
        self.inner.format(TIMESTAMP_FMT).to_string()
    }

    /// returns timestamp prefixing the filepath, if found
    pub fn from_filename_prefix(path: &Path) -> Option<Self> {
        if let Some(file_name) = path.file_name() {
            if let Some(as_str) = file_name.to_str() {
                if as_str.len() >= 16 {
                    let timestamp_str = &as_str[0..15];
                    return TimeStamp::from_str(timestamp_str)
                }
            }
        }
        None
    }

    fn from_str(s: &str) -> Option<Self> {
        NaiveDateTime::parse_from_str(s, TIMESTAMP_FMT).ok().map(|dt| TimeStamp { inner: dt })
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let str = "2020-12-12_12:12";
        let result = TimeStamp::from_str(str);
        assert!(result.is_some());
    }

    #[test]
    fn from_filename_too_short() {
        let path = Path::new("2012");
        let result = TimeStamp::from_filename_prefix(path);
        assert!(result.is_none());
    }

    #[test]
    fn from_start_of_filename() {
        let path = Path::new("2012-12-12_12:12_Some_Tags");
        let result = TimeStamp::from_filename_prefix(path);
        assert!(result.is_some());
    }
}