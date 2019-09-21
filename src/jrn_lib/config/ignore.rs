use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::{JrnError, JrnErrorKind};


#[derive(Debug)]
pub struct JrnIgnore {
    filters: HashSet<String>,
    // used to lazily populate the regex_list from filters
    initialized: bool,
    regex_list: Vec<Regex>,
}

impl Default for JrnIgnore {
    fn default() -> Self {
        JrnIgnore {
            filters: HashSet::new(),
            initialized: true,
            regex_list: Vec::new(),
        }
    }
}

impl JrnIgnore {
    /// read an ignore file from a path
    /// returns empty JrnIgnore if no file is found at path
    /// returning Err if the path can not be read
    /// returns Err if any regex is ill-formatted
    pub fn from_path(path: &Path) -> Result<Self, JrnError> {
        let mut result = JrnIgnore::default();

        if path.exists() {
            let mut buf = String::new();
            let mut file = File::open(path)?;
            file.read_to_string(&mut buf);
            result.filters = buf.lines().map(|s| String::from(s)).collect();
            result.initialized = !result.filters.is_empty();
        }

        &result.init_regex();

        Ok(result)
    }

    /// merge two JrnIgnore objects into one
    pub fn merge(mut self, other: JrnIgnore) -> JrnIgnore {
        for s in other.filters {
            self.filters.insert(s);
            //only need to change init state if other contains any entries
            self.initialized = false;
        }
        self
    }

    /// check rather a path should be ignored
    pub fn ignore(&mut self, path: &Path) -> bool {
        if !self.initialized {
            self.init_regex();
        }
        for r in &self.regex_list {
            if r.is_match(path.to_str().expect("Path is invalid unicode")) {
                return true
            }
        }

        false
    }

    fn init_regex(&mut self) -> Result<(), JrnError> {
        self.regex_list.clear();
        for result in self.filters.iter().map(|s| Regex::new(s)) {
            match result {
                Ok(regex) => self.regex_list.push(regex),
                Err(e) => return Err(JrnError::with_cause(Box::new(e), JrnErrorKind::Regex))
            }
        }
        self.initialized = true;
        Ok(())
    }
}

