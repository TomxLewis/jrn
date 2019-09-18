use std::path::PathBuf;

use chrono::prelude::*;

use super::Config;

/// the in memory representation of a jrn entry
pub struct JrnEntry {
    time: DateTime<Utc>,
    tags: Vec<String>,
    relative_path: PathBuf,
}

impl JrnEntry {
    pub fn new(cfg: &Config) -> Self {
        unimplemented!()
    }
}