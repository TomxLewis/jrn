use super::time::TimeStampFmt;
use super::time::UtcOffset;
use super::error::{JrnError, JrnErrorKind};

use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    editor: String,
    editor_args: Vec<String>,
    timezone: UtcOffset,
    timestamp_fmt: TimeStampFmt,
    tag_start_char: char,
    tag_deliminator: char,
    config_local_tags: Vec<String>,
    ///file path regex's to ignore
    ignore: JrnIgnore,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JrnIgnore {
    filters: HashSet<String>,
}

impl Default for JrnIgnore {
    fn default() -> Self {
        JrnIgnore {
            filters: HashSet::new(),
        }
    }
}

impl JrnIgnore {
    /// read an ignore file from a path
    /// returning Err if the path can not be read
    /// returns empty JrnIgnore if no file is found at path
    ///
    /// TODO return Err if any regex is ill-formatted?
    ///
    pub fn from_path(path: &Path) -> Result<Self, JrnError> {
        let mut filters: HashSet<String> = HashSet::new();

        if path.exists() {
            let mut buf = String::new();
            let mut file = File::open(path)?;
            file.read_to_string(&mut buf);
            filters = buf.lines().map(|s| String::from(s)).collect();
        }

        Ok(
            JrnIgnore {
                filters
            }
        )
    }

    /// merge two JrnIgnore objects into one
    pub fn merge(mut self, other: JrnIgnore) -> JrnIgnore {
        for s in other.filters {
            self.filters.insert(s);
        }
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: String::from("vim"),
            editor_args: vec!(String::from("+star")),
            timezone: UtcOffset::local(),
            timestamp_fmt: TimeStampFmt::Default,
            tag_start_char: '_',
            tag_deliminator: '_',
            config_local_tags: Vec::new(),
            ignore: JrnIgnore::default(),
        }
    }
}

impl Config {
    /// searches the system for a .jrn config file
    /// if not found just returns the default
    ///
    /// specifically will check the following locations and return the most local
    /// in order of global -> local
    ///     ~/.config/.jrnconfig
    ///     ~/.jrnconfig
    ///     ./.jrnconfig
    ///
    pub fn find_or_default() -> Self {
        let mut found_cfg: Option<Config> = None;
        let optional_paths: Vec<Option<PathBuf>> = vec![dirs::config_dir(),
                                                        dirs::home_dir(),
                                                        std::env::current_dir().ok()];

        // filter map possible config directories to config paths
        let paths_to_check: Vec<PathBuf> = optional_paths.into_iter()
            .filter_map(|mut p: Option<PathBuf>| {
                p.map(|mut path_buf: PathBuf| {
                    path_buf.push(String::from(super::JRN_CONFIG_FILE_NAME));
                    path_buf
                })
            })
            .collect();

        // try to read from each, skipping errors, returning the most local found config if any
        for path_buf in paths_to_check {
            match Config::read(&path_buf) {
                Ok(cfg) => found_cfg = Some(cfg),
                _ => {}
            }
        }

        //if we found one return it, else default
        match found_cfg {
            Some(cfg) => cfg,
            None => Config::default()
        }
    }

    /// launches the editor based on the format settings in this config,
    /// returns Err if this fails
    pub fn launch_editor(&self, path: Option<&Path>) -> Result<(), JrnError> {
        let mut args: Vec<String> = Vec::new();
        for arg in self.editor_args.clone() {
            args.push(arg)
        }

        //if no path don't worry
        if let Some(path) = path {
            //check unicode validity of the path if it was given
            let path_str = path.to_str();
            if path_str.is_none() {
                return Err(JrnError::kind(JrnErrorKind::UtfError))
            }
            let path_str = path_str.unwrap();
            args.push(String::from(path_str));
        }

        //build command to send to os
        let cmd = Command::new(&self.editor)
            .args(args)
            .output()?;

        Ok(())
    }

    /// formats the file name for a potential new entry
    /// returning Err if the file already exists
    fn check_path(&self, tags: Option<Vec<&str>>) -> Result<PathBuf, JrnError> {
        let file_name = self.format_file_name(tags);
        let path_buf = PathBuf::from(file_name);

        //return None if entry already exists
        if path_buf.exists() {
            use std::io::{Error, ErrorKind};
            let boxed_err = Box::new(Error::from(ErrorKind::AlreadyExists));
            Err(JrnError::with_cause(boxed_err, JrnErrorKind::IOError))
        }
        else {
            Ok(path_buf)
        }
    }

    /// Read a config file from disk
    /// Returns Err when the file is not readable or properly formatted
    fn read(path: &Path) -> Result<Self, JrnError> {
        use ron::de::from_bytes;

        let mut file = File::open(path)?;
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;
        let cfg = from_bytes(&contents)?;

        Ok(cfg)
    }

    /// formats the file name based on the format settings in this config
    fn format_file_name(&self, tags: Option<Vec<&str>>) -> String {
        let mut file_name = self.timestamp_fmt.get_time_string();

        file_name.push(self.tag_start_char);

        for tag in &self.config_local_tags {
            file_name.push(self.tag_deliminator);
            file_name.push_str(tag);
        }

        if let Some(tags) = tags {
            for tag in tags {
                file_name.push(self.tag_deliminator);
                file_name.push_str(tag);
            }
        }

        file_name
    }

    /// Writes the config struct to a path, truncating any existing file
    /// Returns Err when path is not writable
    fn write(&self, path: &Path) -> Result<(), JrnError> {
        use ron::ser::PrettyConfig;
        use ron::ser::Serializer;
        let mut serializer = Serializer::new(Some(PrettyConfig::default()), true);

        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        self.serialize(&mut serializer)?;

        let serialization_result = serializer.into_output_string();
        file.write_all(&mut serialization_result.as_bytes()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn save_default() {
        let cfg = Config::default();
        let path = PathBuf::from("test.jrn");
        cfg.write(&path);

        let existence = &path.exists();
        std::fs::remove_file(path);

        assert!(existence);
    }

    #[test]
    fn read_default() {
        let cfg = Config::default();
        let path = PathBuf::from("test2.jrn");
        cfg.write(&path);

        let test = Config::read(&path).unwrap();
        std::fs::remove_file(path);

        assert_eq!(cfg, test);
    }

}

