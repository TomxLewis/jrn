use super::entry::JrnEntry;
use super::{IgnorePatterns, Settings, JrnError};
use std::collections::HashMap;
use std::io::Write;
use std::io;
use std::fs::OpenOptions;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
    config: Settings,
    ignore: IgnorePatterns,
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
    pub fn init(config: Settings, ignore: IgnorePatterns) -> Result<Self, JrnError> {
        //TODO
        //list all files in the directory
        //filter all that have valid jrn formatting
        //populate self.entries with found entries
        //populate self.tags with found tags
        let repo = JrnRepo {
            config,
            ignore,
            entries: Vec::new(),
            tags: HashMap::new(),
        };
        Ok(repo)
    }


    /// Tries to create a new entry in this repo
    /// according to the formatting rules in the [Config],
    /// opens the entry in the [Config] editor if requested
    ///
    /// returning Err if failing to create the entry
    pub fn create_entry(&mut self, tags: Option<Vec<String>>, text: Option<&str>, open_editor: bool) -> Result<(), JrnError> {
        let tags = tags.unwrap_or(Vec::new());
        let tags_ref: Vec<&str> = tags.iter().map(|f| f.as_str()).collect();

        let path = self.config.build_path(tags_ref)?;

        if let Some(text) = text {
            let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
            file.write(text.as_bytes())?;
        }

        if open_editor {
            self.config.launch_editor(Some(&path))?;
        }

        Ok(())
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