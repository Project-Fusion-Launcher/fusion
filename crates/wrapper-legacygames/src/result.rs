use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    Reqwest(reqwest::Error),
    Other(String),
}

impl StdError for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Other(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Other(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ParseInt(e) => write!(f, "Parse int error: {}", e),
            Self::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Self::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}
