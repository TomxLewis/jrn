use ron::de::from_bytes;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::BTreeMap;

use super::*;

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
    TagStart,
    TagDeliminator,
    ConfigLocalTags,
}

impl Default for Settings {
    fn default() -> Self {
        use JrnSetting::*;
        let mut map = BTreeMap::new();
        map.insert(Editor, String::from("vim"));
        map.insert(EditorArgs, String::from("+star"));
        map.insert(TagStart, String::from("-"));
        map.insert(TagDeliminator, String::from("_"));
        Settings { map }
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
    /// For Example vim would be used as the editor in the following case
    ///     ~/.jrnconfig
    ///         editor=ed
    ///     ./.jrnconfig
    ///         editor=vim
    ///
    /// If no value is set for a config option the default is used
    ///
    /// This function will not fail, but rather log warnings
    /// these can be used by the applications logger
    pub fn find_or_default() -> Self {
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

        // try to read from each, returning the most local config options if any
        for path_buf in paths_to_check {
            if let Some(found) = Settings::read(&path_buf) {
                working_cfg = working_cfg.merge(found);
            }
        }

        //add any necessary but not found settings from the default and return
        working_cfg.merge(Settings::default())
    }

    pub fn get_tag_deliminator(&self) -> &str {
        self.map.get(&JrnSetting::TagDeliminator).unwrap()
    }

    pub fn get_tag_start(&self) -> &str {
        self.map.get(&JrnSetting::TagStart).unwrap()
    }

    pub fn get_tags(&self) -> Vec<&str> {
        let mut result = Vec::new();
        if let Some(config_tags) = self.map.get(&JrnSetting::ConfigLocalTags) {
            //TODO document need for split char
            let config_tags = config_tags.split(',');
            for tag in config_tags {
                if tag != "" {
                    result.push(tag);
                }
            }
        }
        result
    }

    /// launches the editor based on the settings in this config,
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
        let mut cmd = Command::new(&editor);
        cmd.args(args);

        let mut child = cmd.spawn().map_err(|_| JrnError::EditorNotFound)?;
        child.wait()?;
        Ok(())
    }


    /// convenience method for an empty settings object
    fn empty() -> Self { Settings { map: BTreeMap::new() } }

    /// merge an other into this,
    /// favoring the config settings in self if found in both
    fn merge(mut self, other: Settings) -> Self {
        for (setting, mut value) in other.map {
            match setting {
                //if the setting is ConfigTags, merge
                JrnSetting::ConfigLocalTags => {
                    if !self.map.contains_key(&setting) {
                        self.map.insert(setting, value);
                    }
                    else {
                        //TODO dedupe tags
                        let current = self.map.get(&setting).unwrap();
                        value.push_str(&format!(",{}", current));
                        self.map.insert(setting, value);
                    }
                }
                //else if the setting already exists in self don't overwrite
                _ => {
                    self.map.entry(setting).or_insert(value);
                }
            }
        }
        self
    }

    /// Tries to read self from file path,
    ///
    /// returns None if not found
    /// in the case of IO errors, or configuration formatting errors this function will log warnings
    fn read(path: &Path) -> Option<Self> {
        use log::warn;
        let mut result: Option<Self> = None;

        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut contents: Vec<u8> = Vec::new();
                if file.read_to_end(&mut contents).is_err() {
                    warn!("Can not read from config file: {:?}", path);
                    return None
                }

                if let Ok(serialized) = from_bytes(&contents) {
                    result.replace(serialized);
                }
                else {
                    warn!("Problem reading configuration from path: {:?}\nSkipping", path);
                }
            }
        }

        result
    }

    /// Writes the struct to a path, truncating any existing file
    /// Returns Err when path is not writable
    ///
    /// currently only used for testing
    #[cfg(test)]
    fn write(&self, path: &Path) -> Result<(), JrnError> {
        use std::io::Write;

        let mut serializer = ron::ser::Serializer::new(Some(ron::ser::PrettyConfig::default()), true);
        let mut file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        self.serialize(&mut serializer)?;

        let serialization_result = serializer.into_output_string();
        file.write_all(&serialization_result.as_bytes()).unwrap();
        Ok(())
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_default() {
        let settings = Settings::default();
        let path = PathBuf::from("test.jrnconfig");
        settings.write(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
    }
}

