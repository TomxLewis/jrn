mod ignore;
mod settings;

//imports
use super::*;
use super::entry::JrnEntry;
use super::time::TimeStamp;

//exports
pub use settings::Settings;
pub use ignore::IgnorePatterns;

//statics
static JRN_CONFIG_FILE_NAME: &'static str = ".jrnconfig";
static JRN_IGNORE_FILE_NAME: &'static str = ".jrnignore";

