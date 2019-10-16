use std::fmt::{self, Display, Formatter};
use crate::Settings;

#[derive(Debug, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub struct Location(String);

impl Default for Location {
    fn default() -> Self {
        Location(String::from("None"))
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl From<String> for Location {
    fn from(s: String) -> Self {
        Location(s)
    }
}

impl Location {
    /// TODO-BLOCK
    /// First, if given via commandline, return that
    /// Second, if available in the configuration, return that
    /// Third, if the previous entry had a location, use that
    /// If first entry use [Default]
    pub fn configure(arg: Option<String>, _cfg: &Settings) -> Self {
        if let Some(arg) = arg {
            return Location(arg)
        }
        Location(String::from("None"))
    }
}