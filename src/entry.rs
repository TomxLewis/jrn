use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::{JrnError, Location, Settings, TimeStamp};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// the in memory representation of a jrn entry
#[derive(Debug, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct JrnEntry {
    pub creation_time: TimeStamp,
    pub location: Location,
    pub tags: Vec<String>,
    pub file_path: PathBuf,
}

impl JrnEntry {
    /// Creates and writes a new entry
    pub fn new(
        config: &Settings,
        creation_time: Option<TimeStamp>,
        tags: Option<Vec<String>>,
        location: Option<String>,
    ) -> Self {
        let creation_time = creation_time.unwrap_or_else(TimeStamp::now);
        let tags = tags.unwrap_or_default();
        let location: Location = location
            .map(Location::from)
            .unwrap_or_default(); //TODO pull location from config
        let mut entry = JrnEntry {
            creation_time,
            location,
            tags,
            file_path: PathBuf::new(),
        };
        entry.build_file_path(config);
        entry
    }

    /// Reads an entry from a file path
    pub fn read_entry(path: &Path, config: &Settings) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
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
            "
            )
            .unwrap();
        };

        if let Some(file_path) = path.to_str() {
            if let Some(captures) = RE.captures(file_path) {
                let year: i32 = captures.name("year").unwrap().as_str().parse().unwrap();
                let month: u32 = captures.name("month").unwrap().as_str().parse().unwrap();
                let day: u32 = captures.name("day").unwrap().as_str().parse().unwrap();
                let hr: u32 = captures.name("hr").unwrap().as_str().parse().unwrap();
                let min: u32 = captures.name("min").unwrap().as_str().parse().unwrap();
                let tag_str: &str = captures.name("tags").unwrap().as_str();

                let creation_time = TimeStamp::from_ymdhm(year, month, day, hr, min);
                let tag_delim = config.get_tag_deliminator();
                let tags: Vec<String> = tag_str.split(tag_delim).map(String::from).collect();
                let file_path: PathBuf = PathBuf::from(path);
                let entry = JrnEntry {
                    creation_time,
                    location: Location::default(), //TODO get location from file_path
                    tags,
                    file_path,
                };
                return Some(entry);
            }
        }

        None
    }

    /// Pushes a tag to this entry
    pub fn push_tag(&mut self, tag: &str, config: &Settings) -> std::io::Result<()> {
        self.tags.push(String::from(tag));
        self.update_file_path(config)?;
        Ok(())
    }

    /// Formats this entries file_path as a &str
    pub fn file_path_str(&self) -> &str {
        self.file_path.to_str().unwrap()
    }

    /// Hashes the metadata in this entry, ignoring the files contents
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn build_file_path(&mut self, config: &Settings) {
        let mut file_name = String::new();
        let tag_start = config.get_tag_start();
        let tag_delim = config.get_tag_deliminator();

        //handle time
        file_name.push_str(&self.creation_time.to_string());

        //handle tags
        if !self.tags.is_empty() {
            file_name.push(tag_start);
        }
        let tag_len = self.tags.len();
        for (i, tag) in self.tags.iter().enumerate() {
            file_name.push_str(tag);
            if i < (tag_len - 1) {
                file_name.push(tag_delim);
            }
        }
        let path = PathBuf::from(file_name);
        self.file_path = path;
    }

    fn update_file_path(&mut self, config: &Settings) -> std::io::Result<()> {
        let old = self.file_path.clone();
        self.build_file_path(config);
        std::fs::rename(old, &self.file_path)?;
        Ok(())
    }
}

static DISPLAY_LENGTH: usize = 100;

impl Display for JrnEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use std::iter::{repeat, FromIterator};
        let separator = String::from_iter(repeat('-').take(DISPLAY_LENGTH));

        writeln!(f, "{}", &separator)?;
        writeln!(f, "entry     {:x}", self.get_hash())?;
        writeln!(f, "time      {}", self.creation_time)?;
        writeln!(f, "location  {}", self.location)?;
        write!(f, "tags      ")?;
        for tag in &self.tags {
            write!(f, "{} ", tag)?;
        }
        writeln!(f)?;
        writeln!(f, "{}", &separator)?;

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
