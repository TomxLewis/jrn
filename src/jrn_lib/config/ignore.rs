use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use log::warn;
use regex::Regex;

#[derive(Debug)]
pub struct IgnorePatterns {
    filters: HashSet<String>,
    // used to lazily populate the regex_list from filters
    initialized: bool,
    regex_list: Vec<Regex>,
}

impl Default for IgnorePatterns {
    fn default() -> Self {
        let mut filters = HashSet::new();
        filters.insert(String::from("\\..*"));
        IgnorePatterns {
            filters,
            initialized: false,
            regex_list: Vec::new(),
        }
    }
}

impl IgnorePatterns {
    /// find a Ignore file in the current dir
    /// and use the default ignore patterns
    ///
    /// will log warnings but not fail for IO or Regex Errors
    pub fn find_or_default() -> Self {
        let mut result = IgnorePatterns::default();

        //check the current dir for ignore file
        let current_dir = std::env::current_dir().ok();
        if let Some(mut dir) = current_dir {
            dir.push(super::JRN_IGNORE_FILE_NAME);
            let found = IgnorePatterns::from_path(&dir);
            result = result.merge(found);
        }

        result.init_regex();
        result
    }

    /// returns true if file should be ignored
    pub fn matches(&self, path: &Path) -> bool {
        debug_assert!(self.initialized);

        for r in &self.regex_list {
            if let Some(os_str) = path.file_name() {
                if let Some(p) = os_str.to_str() {
                    if r.is_match(p) {
                        return true
                    }
                } else {
                    warn!("Invalid UTF8, skipping path: {:?}", path);
                }
            }
        }

        false
    }


    /// read an ignore file from a path
    /// returns empty JrnIgnore if no file is found at path
    ///
    /// warns and skips file when
    ///     file cannot be read
    ///     file is not unicode encoded
    fn from_path(path: &Path) -> Self {
        let mut result = IgnorePatterns::empty();

        if path.exists() {
            if let Ok(mut file) = File::open(&path) {
                let mut buf: String = String::new();
                if file.read_to_string(&mut buf).is_ok() {
                    result.filters = buf.lines().map(String::from).collect();
                    result.initialized = !result.filters.is_empty();
                }
                else {
                    warn!("Skipping non-unicode encoded file: {}", path.display());
                }
            }
        }

        result
    }

    /// convenience function for empty Ignore
    fn empty() -> Self {
        IgnorePatterns {
            filters: HashSet::new(),
            initialized: true,
            regex_list: vec![]
        }
    }

    /// merge two patterns into one
    fn merge(mut self, other: IgnorePatterns) -> IgnorePatterns {
        for s in other.filters {
            self.filters.insert(s);
            //only need to change init state if other contains any entries
            self.initialized = false;
        }
        self
    }

    /// builds the regexps in self
    /// logs warnings but will not panic
    fn init_regex(&mut self) {
        self.regex_list.clear();
        for result in self.filters.iter().map(|s| Regex::new(s)) {
            match result {
                Ok(regex) => self.regex_list.push(regex),
                Err(e) => warn!("Invalid Regex\n{}", e)
            }
        }
        self.initialized = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_ignores_git() {
        let path = Path::new(".git");
        let default = IgnorePatterns::find_or_default();
        assert!(default.matches(path));
    }

    #[test]
    fn default_ignores_git_submodule() {
        let path = Path::new("somedir/.git");
        let default = IgnorePatterns::find_or_default();
        assert!(default.matches(path));
    }

    #[test]
    fn default_does_not_ignore_random() {
        let path = Path::new("somedir/should_not_be_ignored");
        let default = IgnorePatterns::find_or_default();
        assert!(!default.matches(path));
    }
}
