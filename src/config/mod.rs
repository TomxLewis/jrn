mod ignore;
mod settings;

//exports
pub use ignore::IgnorePatterns;
pub use settings::Settings;

//statics
static JRN_CONFIG_FILE_NAME: &str = ".jrnconfig";
static JRN_IGNORE_FILE_NAME: &str = ".jrnignore";
