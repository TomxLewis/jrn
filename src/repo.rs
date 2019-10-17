use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use super::*;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
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
        let mut repo = JrnRepo {
            config,
            ignore,
            entries: Vec::new(),
            tags: TagContainer::new(),
        };
        let current_dir: PathBuf =
            std::env::current_dir().expect("jrn needs access to the current working directory");
        repo.collect_entries(&current_dir);
        repo.entries.sort();
        Ok(repo)
    }

    /// Tries to create a new entry in this repo
    ///
    /// returning Err if failing to create the entry
    pub fn create_entry(
        &mut self,
        tags: Vec<String>,
        content: Option<String>,
        skip_edit: bool,
    ) -> Result<(), JrnError> {
        let entry = JrnEntry::new(&self.config, None, tags, None);
        let path = &entry.file_path;

        let mut file = OpenOptions::new().write(true).create(true).open(path)?;

        //create the file
        if let Some(content) = content {
            file.write_all(content.as_bytes())?;
        } else {
            file.write_all(&[])?;
        }

        if !skip_edit {
            self.config.launch_editor(Some(&path));
        }

        self.entries.push(entry);

        Ok(())
    }

    /// display entries to std::out
    /// that match the provided string
    pub fn list_entries(&self, pattern: &str, most_recent: Option<usize>) -> Result<(), JrnError> {
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

    pub fn list_tags(&self) {
        let tags = self.tags.sorted();
        for tag in tags {
            println!("{}: {}", tag.1, tag.0);
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

    /// Helper method to recursively search cd for entries
    /// after calling this method, must preform sorting on entries
    fn collect_entries(&mut self, path: &Path) {
        if self.ignore.matches(path) {
            return;
        }

        if path.is_dir() {
            if let Ok(dir) = fs::read_dir(path) {
                for f in dir {
                    if let Ok(file) = f {
                        self.collect_entries(&file.path());
                    }
                }
            }
        } else if let Some(entry) = JrnEntry::read_entry(path, &self.config) {
            for tag in &entry.tags {
                self.tags.insert(tag);
            }
            self.entries.push(entry);
        }
    }
}
