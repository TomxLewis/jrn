use std::fmt::{self, Display, Formatter};
use crate::JrnRepo;

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
    /// First, return if given via commandline, return that
    /// Second, return if available in the configuration, return that
    /// Third, return the previous entries location
    /// Lastly use [Default]
    pub fn configured_from(arg: Option<String>, repo: &JrnRepo) -> Self {
        if let Some(arg) = arg {
            Location(arg)
        } else if let Some(loc) = repo.get_location() {
            loc
        } else {
            Location::default()
        }
    }
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location(self.0.clone())
    }
}