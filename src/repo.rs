use std::collections::VecDeque;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::*;
use std::ops::Deref;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
    pub root_path: PathBuf,
    config: Settings,
    ignore: IgnorePatterns,
    /// entries sorted by creation time
    entries: Vec<JrnEntry>,
    tags: TagContainer,
}

impl JrnRepo {
    /// Initializes the repo in the current working dir
    ///
    /// This is done by collecting all journal entries in current directory and subdirectories
    /// that fit the settings from the environments config (~/.jrnconfig, ./.jrnconfig ...)
    ///
    /// JrnEntry filenames are formatted
    /// The date and tags are processed from the entries file path yyyy-mm-dd_hhmm{}Tag1{}Tag2{}...TagN
    ///
    /// returning Err if unable to write new entries
    /// will not return Err if unable to read files in dir
    pub fn init(config: Settings, ignore: IgnorePatterns) -> Result<Self, JrnError> {
        let root_path: PathBuf = env::current_dir()
            .expect("jrn needs access to the repository root");

        let mut repo = JrnRepo {
            root_path,
            config,
            ignore,
            entries: Vec::new(),
            tags: TagContainer::new(),
        };
        repo.collect_entries();
        Ok(repo)
    }

    /// Tries to create a new entry in this repo
    pub fn create_entry(
        &mut self,
        tags: Vec<String>,
        location: Option<String>,
        skip_edit: bool,
    ) -> Result<(), JrnError> {
        let entry = JrnEntry::new(&self, None, tags, location);
        let path = &entry.file_path;

        if !skip_edit {
            self.config.launch_editor(Some(&path))?;
        } else {
            let mut file = OpenOptions::new().write(true).create(true).open(path)?;
            //TODO test if line is needed
            file.write_all(&[])?;
        }

        self.entries.push(entry);
        Ok(())
    }

    /// Returns the location to be used by new entries if a location arg was not passed
    /// First returns a location given by the configuration if available
    /// Second returns the previous entries location
    pub fn get_location(&self) -> Option<Location> {
        if let Some(loc) = self.config.get_location() {
            Some(loc)
        } else if let Some(entry) = self.entries.last() {
            Some(entry.location.clone())
        } else {
            None
        }
    }

    /// display entries to std::out
    /// that match the provided string
    pub fn list_entries(&self, pattern: &str, most_recent: Option<usize>) -> Result<(), JrnError> {
        //TODO unify pattern handling between list methods
        let filter = JrnEntryFilter::from_pattern(pattern)?.into_filter();
        let mut matched: VecDeque<&JrnEntry> = self.entries
            .iter()
            .filter(|entry| filter(entry))
            .collect();

        if let Some(most_recent) = most_recent {
            let len = matched.len();
            if most_recent < len {
                for _ in most_recent..len {
                    matched.pop_front();
                }
            }
        }

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        for entry in matched {
            writeln!(handle, "{}", &entry)?;
        }
        Ok(())
    }

    pub fn list_tags(&self, pattern: &str) {
        //TODO unify pattern handling between list methods
        let tags = self.tags.sorted();
        for tag in tags {
            if tag.1.contains(pattern) {
                println!("{}: {}", tag.1, tag.0);
            }
        }
    }

    pub fn push_tag(&mut self, tag: &str, _descriptor: Option<String>) {
        //TODO search for descriptor

        self.tags.insert(tag);
        //push only to the last entry if not specified
        let len = self.entries.len();
        if len > 0 {
            let entry = self.entries.get_mut(len - 1).unwrap();
            if let Err(e) = entry.push_tag(tag, &self.config) {
                self.tags.remove(tag);
                log::error!("{}", e);
            }
        }
    }

    /// Removes the newest entry
    pub fn remove_latest(&mut self) -> io::Result<()> {
        if let Some(removed) = self.entries.pop() {
            for tag in &removed.tags {
                self.remove_tag(tag);
            }
            removed.delete()?;
        }
        Ok(())
    }

    fn remove_tag(&mut self, tag: &str) {
        //TODO implement
        println!("removing tag: {}", tag);
    }

    /// Helper method to walk the filesystem and add entries
    fn collect_entries(&mut self) {
        let path = self.root_path.clone();
        fn collect(repo: &mut JrnRepo, path: &Path) {
            if repo.ignore.matches(path) {
                return;
            }

            if path.is_dir() {
                if let Ok(dir) = fs::read_dir(path) {
                    for f in dir {
                        if let Ok(file) = f {
                            collect(repo, &file.path());
                        }
                    }
                }
            } else if let Some(entry) = JrnEntry::read_entry(path, &repo.config) {
                for tag in &entry.tags {
                    repo.tags.insert(tag);
                }
                repo.entries.push(entry);
            }
        }
        collect(self, &path);
        self.entries.sort();
    }
}

impl Deref for JrnRepo {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}
