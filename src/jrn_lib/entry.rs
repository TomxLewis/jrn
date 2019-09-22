use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use super::Settings;

/// the in memory representation of a jrn entry
#[derive(Debug)]
pub struct JrnEntry {
    tags: Vec<String>,
    relative_path: PathBuf,
}

impl JrnEntry {
    pub fn new(settings: &Settings, tags: Option<Vec<String>>) -> Self {
        unimplemented!()
    }
}

impl std::fmt::Display for JrnEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        //write the filepath
        writeln!(f, "{:?}", self.relative_path)?;

        //write the contents of the file
        let mut file = File::open(&self.relative_path).expect("File Not Found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        writeln!(f, "{}", contents)?;

        Ok(())
    }
}