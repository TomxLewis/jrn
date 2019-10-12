mod ignore;
mod settings;

//imports
use super::*;

//exports
pub use settings::Settings;
pub use ignore::IgnorePatterns;

//statics
static JRN_CONFIG_FILE_NAME: &str = ".jrnconfig";
static JRN_IGNORE_FILE_NAME: &str = ".jrnignore";

