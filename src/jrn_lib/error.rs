type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum JrnError {
    EditorNotFound,
    IO(std::io::Error),
    Serialization(BoxedError),
    InvalidUnicode,
    InvalidRegex,
}

impl std::error::Error for JrnError {}

impl std::fmt::Display for JrnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for JrnError {
    fn from(err: std::io::Error) -> Self {
        JrnError::IO(err)
    }
}

impl From<ron::ser::Error> for JrnError {
    fn from(err: ron::ser::Error) -> Self {
        JrnError::Serialization(Box::new(err))
    }
}

impl From<ron::de::Error> for JrnError {
    fn from(err: ron::de::Error) -> Self {
        JrnError::Serialization(Box::new(err))
    }
}

impl From<regex::Error> for JrnError {
    fn from(err: regex::Error) -> Self {
        JrnError::InvalidRegex
    }
}