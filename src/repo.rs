use std::collections::{VecDeque, BTreeSet};
use std::io::Write;
use std::fs::{self, File, OpenOptions};
use std::path::{PathBuf, Path};

use super::*;

/// in memory knowledge of JrnRepo on disk
pub struct JrnRepo {
    config: Settings,
    ignore: IgnorePatterns,
    /// entries sorted by creation time
    entries: BTreeSet<JrnEntry>,
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
            entries: BTreeSet::new(),
            tags: TagContainer::new(),
        };
        let current_dir: PathBuf = std::env::current_dir().expect("jrn needs access to the current working directory");
        repo.collect_entries(&current_dir);
        Ok(repo)
    }

    /// Tries to create a new entry in this repo
    ///
    /// returning Err if failing to create the entry
    pub fn create_entry(&mut self, tags: Option<Vec<String>>, content: Option<String>, skip_edit: bool) -> Result<(), JrnError> {
        let tags = tags.unwrap_or_default();
        let tags_ref: Vec<&str> = tags.iter().map(|f| f.as_str()).collect();
        let path = self.build_path(tags_ref);
        let file: Option<File> = if content.is_some() || skip_edit { OpenOptions::new().write(true).create(true).open(&path).ok() } else { None };

        if let Some(content) = content {
            file.unwrap().write_all(content.as_bytes())?;
        }
        else if skip_edit {
            //create the file if not launching editor
            file.unwrap().write_all(&[])?;
        }

        if !skip_edit {
            self.config.launch_editor(Some(&path))?;
        }

        Ok(())
    }

    /// display entries to std::out
    /// that match the provided string
    pub fn list_entries(&self, pattern: &str, most_recent: Option<usize>) -> Result<(), JrnError> {
        let filter = JrnEntryFilter::from_pattern(pattern)?.into_filter();

        let mut matched: VecDeque<&JrnEntry> = self.entries.iter().filter(|entry| filter(entry)).collect();
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

    pub fn push_tag(&mut self, tag: &str, num: Option<usize>) {
        //push only to the last entry if not specified
        let num = num.unwrap_or(1);
        //TODO
        //let mut entry_iter = self.entries.iter_mut().rev();

        for _ in 0..num {
            //if let Some(mut entry) = entry_iter.next() {
                //entry.push_tag(&tag);
            //}
        }
    }

    /// formats the file name for a potential new entry
    /// TODO move method to JrnEntry
    fn build_path(&self, tags: Vec<&str>) -> PathBuf {
        let file_name = self.format_file_name(tags);
        PathBuf::from(file_name)
    }

    /// formats the file name based on the format settings in this config
    fn format_file_name(&self, tags: Vec<&str>) -> String {
        let mut file_name = String::new();
        let tag_start_char = self.config.get_tag_start();
        let tag_delim = self.config.get_tag_deliminator();

        //handle time
        let ts = TimeStamp::now();
        let time_string = ts.to_string();
        file_name.push_str(&time_string);

        //gather all tags
        let mut tags = tags.clone();
        tags.append(&mut self.config.get_tags());

        if !tags.is_empty() {
            file_name.push_str(tag_start_char);
        }

        let tag_len = tags.len();
        for (i, tag) in tags.iter().enumerate() {
            file_name.push_str(tag);
            if i < (tag_len - 1) {
                file_name.push_str(tag_delim);
            }
        }

        file_name
    }

    fn collect_entries(&mut self, path: &Path) {
        if self.ignore.matches(path) {
            return
        }

        if path.is_dir() {
            if let Ok(dir) = fs::read_dir(path) {
                for f in dir {
                    if let Ok(file) = f {
                        self.collect_entries(&file.path());
                    }
                }
            }
        }
        else if let Some(entry) = JrnEntry::read_entry(path) {
            for tag in &entry.tags {
                self.tags.insert(tag);
            }
            self.entries.insert(entry);
        }
    }

}
