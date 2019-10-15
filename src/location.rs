use std::fmt::{self, Display, Formatter};

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
