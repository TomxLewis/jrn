mod error;
mod time;
mod config;
mod repo;
mod entry;

//re-exports
//i.e. the structs we want available in main.rs
pub use config::Config;
pub use error::{JrnError, JrnErrorKind};
pub use repo::JrnRepo;

//statics
static JRN_CONFIG_FILE_NAME: &'static str = ".jrnconfig";