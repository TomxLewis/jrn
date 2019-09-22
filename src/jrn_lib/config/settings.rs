use super::*;

use ron::de::from_bytes;
use ron::ser::PrettyConfig;
use ron::ser::Serializer;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Settings {
    #[serde(flatten)]
    map: BTreeMap<JrnSetting, String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Eq, Ord, PartialOrd)]
pub enum JrnSetting {
    Editor,
    EditorArgs,
    TimeZone,
    TimeStampFormat,
    TagStartChar,
    TagDeliminator,
    ConfigLocalTags,
}

impl Default for Settings {
    fn default() -> Self {
        use JrnSetting::*;
        let mut map = BTreeMap::new();
        map.insert(Editor, String::from("vim"));
        map.insert(EditorArgs, String::from("+star"));
        map.insert(TimeZone, String::from("UTC"));
        map.insert(TimeStampFormat, String::from("%Y-%m-%d_%H:%M"));
        map.insert(TagStartChar, String::from("-"));
        map.insert(TagDeliminator, String::from("_"));
        Settings { map }
    }
}

impl Settings {

    /// convenience method for an empty settings object
    fn empty() -> Self { Settings { map: BTreeMap::new() } }

    /// merge an other into this,
    /// favoring the config settings in self if found in both
    pub fn merge(mut self, other: Settings) -> Self {
        for (setting, value) in other.map {
            //does it already exist in self?
            if !self.map.contains_key(&setting) {
                self.map.insert(setting, value);
            }
        }
        self
    }

    /// Tries to read config from file path,
    /// returning Ok(Settings) if found
    /// returns Ok(Settings::empty()) if not found
    ///
    /// returns JrnError on IoError
    pub fn read(path: &Path) -> Result<Self, JrnError> {
        let mut result = Settings::empty();

        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut contents: Vec<u8> = Vec::new();
                file.read_to_end(&mut contents)?;
                //TODO fix impl of from_bytes
                result = from_bytes(&contents)?;
            }
        }

        Ok(result)
    }

    /// Writes the struct to a path, truncating any existing file
    /// Returns Err when path is not writable
    fn write(&self, path: &Path) -> Result<(), JrnError> {
        let mut serializer = Serializer::new(Some(PrettyConfig::default()), true);
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        self.serialize(&mut serializer)?;

        let serialization_result = serializer.into_output_string();
        file.write_all(&mut serialization_result.as_bytes()).unwrap();
        Ok(())
    }

    /// Loads any configuration from the env
    ///
    /// Will check the following locations
    /// in order of global -> local
    ///     ~/.config/.jrnconfig
    ///     ~/.jrnconfig
    ///     ./.jrnconfig
    ///
    /// More local settings will overwrite global settings
    /// For Example:
    ///     ~/.jrnconfig
    ///         editor=ed
    ///     ./.jrnconfig
    ///         editor=vim
    ///
    /// In the above case vim would be used as the editor
    ///
    /// If no value is set for a config option the default is used
    ///
    pub fn find_or_default() -> Result<Self, JrnError> {
        let mut working_cfg: Settings = Settings::empty();
        let optional_paths: Vec<Option<PathBuf>> = vec![dirs::config_dir(),
                                                        dirs::home_dir(),
                                                        std::env::current_dir().ok()];

        // filter map possible config directories to config paths
        let paths_to_check: Vec<PathBuf> = optional_paths.into_iter()
            .filter_map(|p: Option<PathBuf>| {
                p.map(|mut path_buf: PathBuf| {
                    path_buf.push(String::from(super::JRN_CONFIG_FILE_NAME));
                    path_buf
                })
            })
            .collect();

        // try to read from each, propagating errors, returning the most local config options if any
        for path_buf in paths_to_check {
            let found = Settings::read(&path_buf)?;
            working_cfg = working_cfg.merge(found);
        }

        //add any necessary but not found settings from the default
        working_cfg = working_cfg.merge(Settings::default());

        Ok(Settings::from(working_cfg))
    }

    /// launches the editor based on the format settings in this config,
    /// returns Err if this fails
    pub fn launch_editor(&self, path: Option<&Path>) -> Result<(), JrnError> {
        let mut args: Vec<String> = Vec::new();

        //push editor arguments
        //TODO test
        let editor_args = self.map.get(&JrnSetting::EditorArgs).unwrap().split(' ');
        for arg in editor_args {
            args.push(String::from(arg))
        }

        //push path if given
        if let Some(path) = path {
            //don't attempt if path contains invalid unicode
            if let Some(path_str) = path.to_str() {
                args.push(String::from(path_str));
            }
        }

        //build and send command to os
        let editor = self.map.get(&JrnSetting::Editor).unwrap();
        let cmd = Command::new(&editor)
            .args(args)
            .output()?;

        Ok(())
    }

    /// formats the file name for a potential new entry
    /// returning Err if the file already exists
    fn check_path(&self, tags: Option<Vec<&str>>) -> Result<PathBuf, JrnError> {
        let file_name = self.format_file_name(tags);
        let path_buf = PathBuf::from(file_name);

        //return Err if entry already exists
        if path_buf.exists() {
            use std::io::{Error, ErrorKind};
            let boxed_err = Box::new(Error::from(ErrorKind::AlreadyExists));
            Err(JrnError::with_cause(boxed_err, JrnErrorKind::IOError))
        }
        else {
            Ok(path_buf)
        }
    }

    /// formats the file name based on the format settings in this config
    fn format_file_name(&self, tags: Option<Vec<&str>>) -> String {
        let mut file_name = String::new();

        //handle time
        let time_format: TimeStampFmt = self.map.get(&JrnSetting::TimeStampFormat).unwrap().parse().unwrap();
        let time_string = time_format.get_time_string();
        file_name.push_str(&time_string);

        let tag_start_char = self.map.get(&JrnSetting::TagStartChar).unwrap();
        file_name.push_str(tag_start_char);

        let tag_delim = self.map.get(&JrnSetting::TagDeliminator).unwrap();

        if let Some(config_tags) = self.map.get(&JrnSetting::ConfigLocalTags) {
            //TODO accept or document need for split char
            let config_tags = config_tags.split(',');
            for tag in config_local_tags {
                file_name.push_str(tag_delim);
                file_name.push_str(tag);
            }

        }

        if let Some(tags) = tags {
            for tag in tags {
                file_name.push_str(tag_delim);
                file_name.push_str(tag);
            }
        }

        file_name
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_default() {
        let settings = Settings::default();
        let path = PathBuf::from("test.jrnconfig");
        settings.write(&path);
    }
}

