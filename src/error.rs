use std::io;
// use std::path::PathBuf;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub enum CustomError {
    // JsonNotFoundError(PathBuf),
    Io(io::Error),
    JsonParse(serde_json::Error),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // CustomError::JsonNotFoundError(path) => write!(f, "Could not able to find quotes.json. Check it where it is: {:?}", path),
            CustomError::Io(e) => write!(f, "IO error: {}", e),
            CustomError::JsonParse(e) => write!(f, "JSON Parse error: {}", e),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> Self {
        CustomError::Io(e)
    }
}
impl From<serde_json::Error> for CustomError {
    fn from(e: serde_json::Error) -> Self {
        CustomError::JsonParse(e)
    }
}