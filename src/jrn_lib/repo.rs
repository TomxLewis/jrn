use super::entry::JrnEntry;
use super::error::JrnError;
use super::Config;
use std::collections::HashSet;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
    /// entries sorted by creation time
    entries: Vec<JrnEntry>,

    /// unsorted collection of cached tags
    tags: HashSet<String>,
}

impl JrnRepo {
    /// initializes the repo in the current working dir
    /// by collecting all journal entries in files matching [Config] standards
    ///
    /// returning Err if unable to write new entries
    /// will not return Err if unable to read files in dir
    pub fn init(cfg: Config) -> Result<Self, JrnError> {
        unimplemented!()
    }

    /// adds an entry to this repo
    /// according to the formatting rules in the [Config]
    ///
    ///
    /// returning Err if failing for any reason
    pub fn add_entry(&mut self, cfg: Config, tags: Option<Vec<String>>, text: Option<String>) -> Result<(), JrnError> {
        unimplemented!()
    }

}