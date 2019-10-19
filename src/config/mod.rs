mod ignore;
mod settings;

//exports
pub use ignore::IgnorePatterns;
pub use settings::Settings;
use std::fmt::{self, Formatter};

//statics
static JRN_CONFIG_FILE_NAME: &str = ".jrnconfig";
static JRN_IGNORE_FILE_NAME: &str = ".jrnignore";
static DEFAULT_CFG: &str = r"
core.editor=vim
core.viewer=less
core.ignore.file=.gitignore
core.user=
auth.token=
auth.method=
";

impl Default for Config {
    fn default() -> Self {
        DEFAULT_CFG.parse::<NaiveConfig>().ok().unwrap().into_scoped(ConfigScope::Default)
    }
}

#[derive(Debug)]
/// Type that holds onto all configuration values for the application.
/// The associated methods of this type will always return the value in the closest scope
pub struct Config {
    /// Vec which acts as a set on Scope/Key Pairs
    inner: Vec<ScopedConfigEntry>
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Exists to order all configuration values
/// from farthest scope < nearest scope
pub enum ConfigScope {
    Default,
    System,
    User,
    Local,
}

#[derive(Debug)]
pub enum ConfigKey {
    CoreEditor,
    CoreViewer,
    CoreIgnoreFile,
    CoreUser,
    CoreEmail,
    AuthToken,
    AuthMethod,
}

#[derive(Debug)]
struct ScopedConfigEntry {
    scope: ConfigScope,
    key: ConfigKey,
    value: ConfigValue,
}

/// A config struct without any scope attached to it,
/// could be parsed from a file
struct NaiveConfig {
    inner: Vec<NaiveConfigEntry>
}

struct NaiveConfigEntry {
    key: ConfigKey,
    value: ConfigValue,
}

#[derive(Debug)]
pub enum ConfigParseError {
    NoDeliminator,
    UnknownMeaning(String),
}

type ConfigValue = Option<String>;

impl fmt::Display for ConfigKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use ConfigKey::*;
        match &self {
            CoreEditor => write!(f, "core.editor"),
            CoreViewer => write!(f, "core.viewer"),
            CoreIgnoreFile => write!(f, "core.ignore.file"),
            CoreUser => write!(f, "core.user"),
            CoreEmail => write!(f, "core.email"),
            AuthToken => write!(f, "auth.token"),
            AuthMethod => write!(f, "auth.method"),
        }
    }
}

impl std::str::FromStr for ConfigKey {
    type Err = ConfigParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfigKey::*;
        match s {
            "core.editor" => Ok(CoreEditor),
            "core.viewer" => Ok(CoreViewer),
            "core.ignore.file" => Ok(CoreIgnoreFile),
            "core.user" => Ok(CoreUser),
            "core.email" => Ok(CoreEmail),
            "auth.token" => Ok(AuthToken),
            "auth.method" => Ok(AuthMethod),
            _ => Err(ConfigParseError::UnknownMeaning(s.to_string()))
        }
    }
}

impl std::str::FromStr for NaiveConfigEntry {
    type Err = ConfigParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let key_str = split.next().ok_or(ConfigParseError::NoDeliminator)?;
        let key: ConfigKey = key_str.parse()?;
        let value: ConfigValue = if let Some(s) = split.next() { if s == "" { None } else { Some(s.to_string()) }} else { None };
        Ok(NaiveConfigEntry{
            key,
            value,
        })
    }
}

impl From<Vec<NaiveConfigEntry>> for NaiveConfig {
    fn from(inner: Vec<NaiveConfigEntry>) -> Self {
        NaiveConfig {
            inner
        }
    }
}

impl std::str::FromStr for NaiveConfig {
    type Err = ConfigParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .map(|s| s.parse::<NaiveConfigEntry>())
            .filter_map(|r| r.ok())
            .collect::<Vec<NaiveConfigEntry>>()
            .into()
        )
    }
}

trait IntoScoped<T> {
    fn into_scoped(self, scope: ConfigScope) -> T;
}

impl IntoScoped<Config> for NaiveConfig {
    fn into_scoped(self, scope: ConfigScope) -> Config {
        Config {
            inner: self.inner.into_iter().map(|n| n.into_scoped(scope)).collect()
        }
    }
}

impl IntoScoped<ScopedConfigEntry> for NaiveConfigEntry {
    fn into_scoped(self, scope: ConfigScope) -> ScopedConfigEntry {
        ScopedConfigEntry {
            scope,
            key: self.key,
            value: self.value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scope_order() {
        assert!(ConfigScope::Local > ConfigScope::User);
        assert!(ConfigScope::User > ConfigScope::System);
        assert!(ConfigScope::System > ConfigScope::Default);
    }
}