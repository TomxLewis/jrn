type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct JrnError {
    pub kind: JrnErrorKind,
    file: &'static str,
    line: u32,
    cause: Option<BoxedError>,
}

#[derive(Debug)]
pub enum JrnErrorKind {
    IOError,
    UtfError,
    Regex,
}

impl std::error::Error for JrnError {}

impl std::fmt::Display for JrnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "kind: {:?}\nfile: {}\nline: {}\ncause: {:?}", self.kind, self.file, self.line, self.cause)
    }
}

impl JrnError {

    #[inline]
    pub fn with_cause(error: BoxedError, kind: JrnErrorKind) -> Self {
        JrnError::build(Some(error), kind)
    }

    #[inline]
    pub fn kind(kind: JrnErrorKind) -> Self {
        JrnError::build(None, kind)
    }

    #[inline]
    fn build(cause: Option<BoxedError>, kind: JrnErrorKind) -> Self {
        JrnError {
            kind,
            file: file!(),
            line: line!(),
            cause,
        }
    }

    pub fn into_cause(self) -> Option<BoxedError> {
        self.cause
    }
}

impl From<std::io::Error> for JrnError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        JrnError::build(Some(Box::new(err)), JrnErrorKind::IOError)
    }
}

impl From<ron::ser::Error> for JrnError {
    #[inline]
    fn from(err: ron::ser::Error) -> Self {
        JrnError::build(Some(Box::new(err)), JrnErrorKind::IOError)
    }
}

impl From<ron::de::Error> for JrnError {
    #[inline]
    fn from(err: ron::de::Error) -> Self {
        JrnError::build(Some(Box::new(err)), JrnErrorKind::IOError)
    }
}