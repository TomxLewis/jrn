use serde::{Serialize, Deserialize};
use super::time::TimeStampFmt;
use super::time::UtcOffset;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    editor: String,
    editor_args: String,
    timezone: UtcOffset,
    timestamp_fmt: TimeStampFmt,
    tag_start_char: char,
    tag_deliminator: char,
    config_local_tags: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: String::from("vim"),
            editor_args: String::from("+star"),
            timezone: UtcOffset::local(),
            timestamp_fmt: TimeStampFmt::Default,
            tag_start_char: '_',
            tag_deliminator: '_',
            config_local_tags: Vec::new(),
        }
    }
}

use super::error::JrnError;

use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{Read, Write};

impl Config {

    /// searches the system for a .jrn config file
    /// if not found creates the default
    pub fn find_or_default() -> Self {
        //TODO search system
        Config::default()
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

    pub fn new_entry(&self, _tags: Option<Vec<&str>>) {
        println!("New Entry");
    }


    pub fn launch_editor(&self, path: &Path) -> Result<(), JrnError> {
        //check unicode validity of path
        let path_str = path.to_str();
        if path_str.is_none() {
            return Err(JrnError::with_msg("path contains invalid UTF8"))
        }
        let path_str = path_str.unwrap();
        dbg!(&path_str);

        //build command to send to os
        let cmd = Command::new(&self.editor)
            .arg(&self.editor_args)
            .arg(path_str)
            .output();

        //check result of command and build an error if needed
        if cmd.is_err() {
            let err_msg = format!("Failed to start process: {} {} {}", self.editor, self.editor_args, path_str);
            return Err(JrnError::from(err_msg))
        }

        Ok(())
    }

    /// formats the file name based on the format settings in the config
    fn format_file_name(&self, tags: Vec<&str>) -> String {
        let mut file_name = self.timestamp_fmt.get_time_string();

        file_name.push(self.tag_start_char);

        for tag in &self.config_local_tags {
            file_name.push(self.tag_deliminator);
            file_name.push_str(tag);
        }

        for tag in tags {
            file_name.push(self.tag_deliminator);
            file_name.push_str(tag);
        }

        file_name
    }

    /// builds up a file name and path for a potential new entry
    /// returning None if the file already exists
    fn check_path(&self, tags: Vec<&str>) -> Option<PathBuf> {
        let file_name = self.format_file_name(tags);
        let path_buf = PathBuf::from(file_name);

        //return None if entry already exists
        if path_buf.exists() {
            None
        }
        else {
            Some(path_buf)
        }
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

        assert!(path.exists());
        std::fs::remove_file(path);
    }

    #[test]
    fn read_default() {
        let cfg = Config::default();
        let path = PathBuf::from("test2.jrn");
        cfg.write(&path);

        let test = Config::read(&path).unwrap();
        assert_eq!(cfg, test);

        std::fs::remove_file(path);
    }

}

