type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct JrnError {
    msg: Option<String>,
    file: &'static str,
    line: u32,
    cause: Option<BoxedError>,
}

impl std::error::Error for JrnError {}

impl std::fmt::Display for JrnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "msg: {:?}\nfile: {}\nline: {}\ncause: {:?}", self.msg, self.file, self.line, self.cause)
    }
}

impl JrnError {
    #[inline]
    pub fn with_msg(msg: &str) -> Self {
        JrnError::build(Some(msg), None)
    }

    #[inline]
    pub fn with_cause(error: BoxedError) -> Self {
        JrnError::build(None, Some(error))
    }

    #[inline]
    fn build(msg: Option<&str>, cause: Option<BoxedError>) -> Self {
        JrnError {
            msg: msg.map(|s| String::from(s)),
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
        let msg = format!("IO Error: {:?}", &err.kind());
        JrnError::build(Some(&msg), Some(Box::new(err)))
    }
}

impl From<ron::ser::Error> for JrnError {
    #[inline]
    fn from(err: ron::ser::Error) -> Self {
        let msg = format!("Serialization Error: {:?}", &err);
        JrnError::build(Some(&msg), Some(Box::new(err)))
    }
}

impl From<ron::de::Error> for JrnError {
    #[inline]
    fn from(err: ron::de::Error) -> Self {
        let msg = format!("Deserialization Error: {:?}", &err);
        JrnError::build(Some(&msg), Some(Box::new(err)))
    }
}