mod config;
mod ignore;
mod io;

//imports
use super::*;
use super::time::UtcOffset;
use super::time::TimeStampFmt;

//exports
pub use ignore::JrnIgnore;
pub use config::Config;

//statics
static JRN_CONFIG_FILE_NAME: &'static str = ".jrnconfig";

