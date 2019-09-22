use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use crate::{JrnError, JrnErrorKind};


#[derive(Debug)]
pub struct IgnorePatterns {
    filters: HashSet<String>,
    // used to lazily populate the regex_list from filters
    initialized: bool,
    regex_list: Vec<Regex>,
}

impl Default for IgnorePatterns {
    fn default() -> Self {
        IgnorePatterns {
            filters: HashSet::new(),
            initialized: true,
            regex_list: Vec::new(),
        }
    }
}

impl IgnorePatterns {
    /// read an ignore file from a path
    /// returns empty JrnIgnore if no file is found at path
    pub fn from_path(path: &Path) -> Self {
        let mut result = IgnorePatterns::default();

        if path.exists() {
            if let Ok(mut file) = File::open(&path) {
                let mut buf: String = String::new();
                file.read_to_string(&mut buf).expect(&format!("Invalid utf in ignore file: {}", path.display()));
                result.filters = buf.lines().map(|s| String::from(s)).collect();
                result.initialized = !result.filters.is_empty();
            }
        }

        result
    }

    /// merge two patterns into one
    pub fn merge(mut self, other: IgnorePatterns) -> IgnorePatterns {
        for s in other.filters {
            self.filters.insert(s);
            //only need to change init state if other contains any entries
            self.initialized = false;
        }
        self
    }

    /// check rather a path should be ignored
    /// this returns Err only on upon being initialized with invalid regex
    pub fn ignore(&mut self, path: &Path) -> Result<bool, JrnError> {
        if !self.initialized {
            self.init_regex()?;
        }
        for r in &self.regex_list {
            if r.is_match(path.to_str().expect("Path is invalid unicode")) {
                return Ok(true)
            }
        }

        Ok(false)
    }

    /// builds the regexps in self
    /// returning Err on failing to do so
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

