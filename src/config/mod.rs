mod ignore;
mod settings;

pub use ignore::IgnorePatterns;
pub use settings::Settings;
use std::fmt::{self, Formatter};
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read};

static JRN_CONFIG_FILE_NAME: &str = ".jrnconfig";
static JRN_IGNORE_FILE_NAME: &str = ".jrnignore";

#[derive(Debug)]
/// Type that holds onto all configuration values for the application.
/// The associated methods of this type will always return the value in the closest scope
pub struct Config {
    /// Vec which acts as a set on Scope/Key Pairs
    inner: Vec<ScopedConfigEntry>
}

impl Config {
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
        let mut result = Config { inner: vec![] };
        use ConfigScope::*;
        static SCOPES: [ConfigScope; 4] = [Default, System, User, Local];
        for scope in SCOPES.iter() {
            if let Some(cfg) = scope.get_config() {
                result = result.merge(cfg);
            }
        }
        result
    }

    pub fn get_editor(&self) -> &str {
        // safe to unwrap as default is always created
        &self.get_value(ConfigKey::CoreEditor).as_ref().unwrap()
    }

    fn get_value(&self, key: ConfigKey) -> &ConfigValue {
        let mut result: &ConfigValue = &None;
        let mut scope: &ConfigScope = &ConfigScope::Default;
        for e in self.inner.iter().filter(|e| e.key == key) {
            if &e.scope >= scope {
                scope = &e.scope;
                result = &e.value;
            }
        }
        result
    }

    fn merge(self, other: Self) -> Self {
        let mut inner = self.inner;
        let mut other = other.inner;
        inner.append(&mut other);
        Config {
            inner
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        use ConfigKey::*;
        NaiveConfig {
            inner: vec![
                NaiveConfigEntry::new(CoreEditor, Some("vim")),
                NaiveConfigEntry::new(CoreViewer, Some("less")),
                NaiveConfigEntry::new(CoreIgnoreFile, Some(".gitignore")),
                NaiveConfigEntry::new(CoreUser, None),
                NaiveConfigEntry::new(CoreEmail, None),
                NaiveConfigEntry::new(AuthToken, None),
                NaiveConfigEntry::new(AuthMethod, None),
            ]
        }
            .into_scoped(ConfigScope::Default)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Exists to order all configuration values
/// from farthest scope < nearest scope
pub enum ConfigScope {
    Default,
    System,
    User,
    Local,
}

impl ConfigScope {
    pub fn get_config(self) -> Option<Config> {
        match self {
            ConfigScope::Default => Some(Config::default()),
            _ => self.read_optionally()
        }
    }

    fn read_optionally(self) -> Option<Config> {
        match self.read() {
            Ok(c) => Some(c),
            Err(e) => { log::trace!("{} Can not read {:?} config, will skip.", e, &self); None }
        }
    }

    fn read(self) -> Result<Config, io::Error> {
        let path = self.get_path().unwrap();
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let cfg = content.parse::<NaiveConfig>().ok().unwrap(); //TODO test if parsing can fail
        Ok(cfg.into_scoped(self))
    }

    fn get_path(self) -> Option<PathBuf> {
        use ConfigScope::*;
        let path_buf = match self {
            Default => None,
            System => dirs::config_dir(), //TODO define real system config path
            User => dirs::home_dir(),
            Local => std::env::current_dir().ok(),
        };
        path_buf.map(|mut pb| { pb.push(JRN_CONFIG_FILE_NAME); pb })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(PartialEq)]
struct NaiveConfigEntry {
    key: ConfigKey,
    value: ConfigValue,
}

impl NaiveConfigEntry {
    fn new(key: ConfigKey, value: Option<&str>) -> Self {
        NaiveConfigEntry {
            key,
            value: value.map(String::from),
        }
    }
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
        let s = s.trim();
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
        let value: ConfigValue = if let Some(s) = split.next() {
            let s = s.trim();
            if s == "" {
                None
            } else {
                Some(s.to_string())
            }
        } else {
            None
        };
        Ok(NaiveConfigEntry{
            key,
            value,
        })
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

impl From<Vec<NaiveConfigEntry>> for NaiveConfig {
    fn from(inner: Vec<NaiveConfigEntry>) -> Self {
        NaiveConfig {
            inner
        }
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

    #[test]
    fn parse_str() {
        let s = r"
        core.editor=vim
        fake.entry=nonsense
        ";

        let c = s.parse::<NaiveConfig>().unwrap();
        let expected = NaiveConfigEntry::new(ConfigKey::CoreEditor, Some("vim"));
        assert_eq!(c.inner.len(), 1);
        assert!(c.inner.contains(&expected));
    }
}