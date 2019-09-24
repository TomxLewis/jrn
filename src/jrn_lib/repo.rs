use super::entry::JrnEntry;
use super::{IgnorePatterns, Settings, TimeStamp, JrnError};
use std::collections::HashMap;
use std::io::Write;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::path::{PathBuf, Path};

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
        //list all files in the directory that are not ignored
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

        let path = self.build_path(tags_ref);

        let mut file: Option<File> = None;

        if text.is_some() || !open_editor {
            //create the file
            let f = OpenOptions::new().write(true).create(true).open(&path)?;
            file = Some(f);
        }

        if let Some(text) = text {
            file.unwrap().write(text.as_bytes())?;
        }
        else if !open_editor {
            //create the file if not launching editor
            file.unwrap().write(&[])?;
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
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        for entry in &self.entries {
            if filter(&entry) {
                writeln!(handle, "{}", &entry)?;
            }
        }
        Ok(())
    }

    /// reads an entry from a path, propagating errors
    pub fn read_entry(path: &Path) -> Result<JrnEntry, JrnError> {
        let str = path.to_str().ok_or(JrnError::InvalidUnicode)?;
        unimplemented!()
    }

    /// formats the file name for a potential new entry
    fn build_path(&self, tags: Vec<&str>) -> PathBuf {
        let file_name = self.format_file_name(tags);
        let path_buf = PathBuf::from(file_name);
        path_buf
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

        for tag in tags {
            file_name.push_str(tag_delim);
            file_name.push_str(tag);
        }

        file_name
    }

    fn visit_recursively(&self, dir: &Path, f: &impl Fn(&DirEntry)) {
        if self.ignore.matches(dir) {
            return
        }
        
        if dir.is_dir() {
            for entry in fs::read_dir(dir) {
            }
        }
    }
}
