use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    DieselConnection(diesel::ConnectionError),
    Io(io::Error),
    Tauri(tauri::Error),
    Other(String),
}

impl StdError for Error {}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::Diesel(e)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(e: diesel::ConnectionError) -> Self {
        Self::DieselConnection(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<tauri::Error> for Error {
    fn from(e: tauri::Error) -> Self {
        Self::Tauri(e)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(e: Box<dyn StdError + Send + Sync>) -> Self {
        Self::Other(e.to_string())
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
            Self::Diesel(e) => write!(f, "Diesel error: {}", e),
            Self::DieselConnection(e) => write!(f, "Diesel connection error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Tauri(e) => write!(f, "Tauri error: {}", e),
            Self::Other(e) => write!(f, "Error: {}", e),
        }
    }
}
