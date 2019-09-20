type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct JrnError {
    file: &'static str,
    line: u32,
    cause: Option<BoxedError>,
    pub kind: JrnErrorKind,
}

#[derive(Debug)]
pub enum JrnErrorKind {
    IOError,
    UtfError,
}

impl std::error::Error for JrnError {}

impl std::fmt::Display for JrnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "msg: {:?}\nfile: {}\nline: {}\ncause: {:?}", self.msg, self.file, self.line, self.cause)
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
            file: file!(),
            line: line!(),
            cause,
            kind,
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