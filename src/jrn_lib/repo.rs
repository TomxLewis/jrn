use super::entry::JrnEntry;
use super::error::JrnError;
use super::Settings;
use std::collections::HashMap;
use std::io::Write;
use std::io;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
    /// our current config
    config: Settings,

    /// entries sorted by creation time
    entries: Vec<JrnEntry>,

    /// unsorted collection of cached tags, mapped to the number of times they appear
    tags: HashMap<String, u16>,
}

impl JrnRepo {
    /// initializes the repo in the current working dir
    /// by collecting all journal entries in files matching [Config] standards
    /// and reading their Tags
    ///
    /// returning Err if unable to write new entries
    /// will not return Err if unable to read files in dir
    pub fn init(config: Settings) -> Result<Self, JrnError> {
        //TODO
        //list all files in the directory
        //filter all that have valid jrn formatting
        //populate self.entries with found entries
        //populate self.tags with found tags
        let repo = JrnRepo {
            config,
            entries: Vec::new(),
            tags: HashMap::new(),
        };
        Ok(repo)
    }


    /// Tries to create a new entry in this repo
    /// according to the formatting rules in the [Config],
    /// opens the entry in the [Config] editor.
    ///
    /// returning Err if failing to create the entry
    pub fn create_entry(&mut self, tags: Option<Vec<String>>, text: Option<String>) -> Result<(), JrnError> {
        //gather all tags
        //

        unimplemented!()
    }

    /// opens an entry in the cfg specified editor
    ///
    /// returning Err if the editor fails to start
    pub fn open_entry(&self, entry: Option<&JrnEntry>) -> Result<(), JrnError> {
        unimplemented!()
    }

    pub fn modify_tags(&mut self, entry: &mut JrnEntry, tags: Option<Vec<String>>) -> Result<(), JrnError> {
        //determine added tags
        //determine removed tags
        //update entry tags on disk entry
        //update entry tags on cached entry
        //update the number of times an entry appears in self.tags
        unimplemented!()
    }

    /// display entries to std::out
    /// that match a provided filter
    pub fn display_entries(&self, filter: &impl Fn(&JrnEntry) -> bool) -> Result<(), JrnError> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for entry in &self.entries {
            if filter(&entry) {
                writeln!(handle, "{}", &entry)?;
            }
        }
        Ok(())
    }
}