use super::*;

use ron::de::from_bytes;
use ron::ser::PrettyConfig;
use ron::ser::Serializer;
use serde::{Serialize, Deserialize, Deserializer};
use serde::Serializer as SerdeSerializer;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Settings {
    editor: String,
    editor_args: Vec<String>,
    timezone: UtcOffset,
    timestamp_fmt: TimeStampFmt,
    tag_start_char: char,
    tag_deliminator: char,
    config_local_tags: Vec<String>,
}

// TODO Manually implement Serialize and Deserialize
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(default)]
pub struct SettingsFileResults {
    editor: Option<String>,
    editor_args: Option<Vec<String>>,
    timezone: Option<UtcOffset>,
    timestamp_fmt: Option<TimeStampFmt>,
    tag_start_char: Option<char>,
    tag_deliminator: Option<char>,
    config_local_tags: Option<Vec<String>>,
}

//impl<'de> Deserialize<'de> for OptionalConfig {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
//        D: Deserializer<'de> {
//
//        #[derive(Deserialize)]
//        #[serde(field_identifier, rename_all = "snake_case")]
//        enum Field {
//            Editor,
//            EditorArgs,
//            Timezone,
//            TimestampFmt,
//            TagStartChar,
//            TagDeliminator,
//            ConfigLocalTags
//        }
//
//    }
//}

impl Default for SettingsFileResults {
    fn default() -> Self {
        SettingsFileResults {
            editor: None,
            editor_args: None,
            timezone: None,
            timestamp_fmt: None,
            tag_start_char: None,
            tag_deliminator: None,
            config_local_tags: None,
        }
    }
}

impl From<SettingsFileResults> for Settings {
    fn from(opt: SettingsFileResults) -> Self {
        Settings {
            editor: opt.editor.unwrap_or(String::from("vim")),
            editor_args: opt.editor_args.unwrap_or(vec!(String::from("+star"))),
            timezone: opt.timezone.unwrap_or(UtcOffset::local()),
            timestamp_fmt: opt.timestamp_fmt.unwrap_or_default(),
            tag_start_char: opt.tag_start_char.unwrap_or('-'),
            tag_deliminator: opt.tag_deliminator.unwrap_or('_'),
            config_local_tags: opt.config_local_tags.unwrap_or(Vec::new()),
        }
    }
}

/// confusing cause changes you want to make here
/// would instead be made in From<SettingsFileResults> for Settings
impl Default for Settings {
    fn default() -> Self {
        let opt = SettingsFileResults::default();
        Settings::from(opt)
    }
}

impl SettingsFileResults {
    /// merge an other OptionalConfig,
    /// favoring the config settings in self if found in both
    pub fn merge(self, other: SettingsFileResults) -> Self {
        SettingsFileResults {
            editor: self.editor.or(other.editor),
            editor_args: self.editor_args.or(other.editor_args),
            timezone: self.timezone.or(other.timezone),
            timestamp_fmt: self.timestamp_fmt.or(other.timestamp_fmt),
            tag_start_char: self.tag_start_char.or(other.tag_start_char),
            tag_deliminator: self.tag_deliminator.or(other.tag_deliminator),
            config_local_tags: self.config_local_tags.or(other.config_local_tags),
        }
    }

    /// Tries to read config from file path,
    /// returning Ok(SettingsFileResults::default()) if file not found,
    /// essentially None
    ///
    /// returning JrnError on IoError
    ///
    /// '''
    /// let non_existent_file = PathBuf::from("fake_file_name")
    /// let return = SettingsFileResults::read_or_default(&non_existent_file);
    /// assert!(return.is_ok());
    /// '''
    ///
    pub fn read_or_default(path: &Path) -> Result<Self, JrnError> {
        let mut result = SettingsFileResults::default();

        if path.exists() {
            let mut file = File::open(path)?;
            let mut contents: Vec<u8> = Vec::new();
            file.read_to_end(&mut contents)?;
            result = from_bytes(&contents)?;
        }

        Ok(result)
    }

    /// Writes the config struct to a path, truncating any existing file
    /// Returns Err when path is not writable
    fn write(&self, path: &Path) -> Result<(), JrnError> {
        let mut serializer = Serializer::new(Some(PrettyConfig::default()), true);
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        self.serialize(&mut serializer)?;

        let serialization_result = serializer.into_output_string();
        file.write_all(&mut serialization_result.as_bytes()).unwrap();
        Ok(())
    }
}

impl Settings {
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
        let mut working_cfg: SettingsFileResults = SettingsFileResults::default();
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

        // try to read from each, propagating errors, returning the most local config options if any
        for path_buf in paths_to_check {
            let found = SettingsFileResults::read_or_default(&path_buf)?;
            working_cfg = working_cfg.merge(found);
        }

        //if we found one return it, else default
        Ok(Settings::from(working_cfg))
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
}

#[cfg(test)]
mod test {
    use super::*;

}

