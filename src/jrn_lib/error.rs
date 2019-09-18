type BoxedError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct JrnError {
    msg: String,
    file: &'static str,
    line: u32,
}

impl std::error::Error for JrnError {}

impl std::fmt::Display for JrnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}\nfile: {}\nline: {}", self.msg, self.file, self.line)
    }
}

impl JrnError {
    #[inline]
    pub fn with_msg(msg: &str) -> Self {
        JrnError {
            msg: String::from(msg),
            file: file!(),
            line: line!(),
        }
    }
}

impl From<String> for JrnError {
    #[inline]
    fn from(msg: String) -> Self {
        JrnError {
            msg,
            file: file!(),
            line: line!(),
        }
    }
}

impl From<std::io::Error> for JrnError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        let msg = format!("IO Error: {:?}", &err.kind());
        JrnError::from(msg)
    }
}

impl From<ron::ser::Error> for JrnError {
    #[inline]
    fn from(err: ron::ser::Error) -> Self {
        let msg = format!("Serialization Error: {:?}", &err);
        JrnError::from(msg)
    }
}

impl From<ron::de::Error> for JrnError {
    #[inline]
    fn from(err: ron::de::Error) -> Self {
        let msg = format!("Deserialization Error: {:?}", &err);
        JrnError::from(msg)
    }
}