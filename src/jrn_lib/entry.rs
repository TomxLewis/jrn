use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use regex::Regex;
use lazy_static::lazy_static;

use super::{JrnError, TimeStamp};

/// the in memory representation of a jrn entry
#[derive(Debug, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct JrnEntry {
    pub creation_time: TimeStamp,
    pub tags: Vec<String>,
    pub file_path: PathBuf,
}

impl JrnEntry {

    pub fn file_path_str(&self) -> &str {
        self.file_path.to_str().unwrap()
    }

    /// tries to reads an entry from a path, if possible
    pub fn read_entry(path: &Path) -> Option<Self> {

        lazy_static!(
            static ref RE: Regex = Regex::new(r"(?x)
            (?P<year>\d{4})
            -
            (?P<month>\d{2})
            -
            (?P<day>\d{2})
            _
            (?P<hr>\d{2})
            (?P<min>\d{2})
            -?
            (?P<tags>.*)?
            ").unwrap();
        );

        if let Some(filename) = path.to_str() {
            if let Some(captures) = RE.captures(filename) {
                let year: i32 = captures.name("year").unwrap().as_str().parse().unwrap();
                let month: u32 = captures.name("month").unwrap().as_str().parse().unwrap();
                let day: u32 = captures.name("day").unwrap().as_str().parse().unwrap();
                let hr: u32 = captures.name("hr").unwrap().as_str().parse().unwrap();
                let min: u32 = captures.name("min").unwrap().as_str().parse().unwrap();
                let tag_str: &str = captures.name("tags").unwrap().as_str();

                let creation_time = TimeStamp::from_ymdhm(year, month, day, hr, min);
                let tags: Vec<String> = tag_str.split('_').map(String::from).collect();
                let file_path: PathBuf = PathBuf::from(path.file_name().unwrap().to_str().unwrap());
                let entry = JrnEntry {
                    creation_time,
                    tags,
                    file_path,
                };
                return Some(entry)
            }
        }

        None
    }

    /// Pushes a tag to this entry if possible
    pub fn push_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.tags.push(String::from(tag));
        
        //TODO place the tag in the entries contents or/and filename
        
        Ok(())
    }

    pub fn remove_tag(&mut self, tag: &str) -> std::io::Result<()> {

        Ok(())
    }
}

impl std::fmt::Display for JrnEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let fps = self.file_path_str();
        //write the filepath
        writeln!(f, "{}", fps)?;
        for _ in 0..fps.chars().count() {
            write!(f, "-")?;
        }
        writeln!(f)?;

        //write the contents of the file
        let mut file = File::open(&self.file_path).expect("File Not Found");
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
    pub fn into_filter(self) -> Box<impl Fn(&JrnEntry) -> bool> {
        Box::new(move |entry: &JrnEntry| self.regex.is_match(&entry.file_path_str()))
    }

    pub fn from_pattern(re: &str) -> Result<Self, JrnError> {
        let filter = JrnEntryFilter {
            regex: Regex::new(re)?,
        };
        Ok(filter)
    }
}
