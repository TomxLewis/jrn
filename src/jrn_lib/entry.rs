use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

use super::Settings;
use regex::Regex;
use crate::JrnError;

/// the in memory representation of a jrn entry
#[derive(Debug)]
pub struct JrnEntry {
    tags: Vec<String>,
    pub relative_path: PathBuf,
}

impl JrnEntry {
    pub fn from(settings: &Settings, path: &Path) -> Self {
        unimplemented!()
    }
}

impl std::fmt::Display for JrnEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        //write the filepath
        writeln!(f, "{:?}", self.relative_path)?;

        //write the contents of the file
        let mut file = File::open(&self.relative_path).expect("File Not Found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        writeln!(f, "{}", contents)?;

        Ok(())
    }
}

pub struct JrnEntryFilter {
    regex: Regex,
}

impl JrnEntryFilter {
    pub fn into_filter(self) -> Box<Fn(&JrnEntry) -> bool> {
        Box::new(move |entry: &JrnEntry| self.regex.is_match(entry.relative_path.to_str().unwrap()))
    }

    pub fn from_pattern(re: &str) -> Result<Self, JrnError> {
        let filter = JrnEntryFilter {
            regex: Regex::new(re)?,
        };
        Ok(filter)
    }
}
