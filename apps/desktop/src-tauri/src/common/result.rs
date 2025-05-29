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
    JoinError(tokio::task::JoinError),
    AcquireError(tokio::sync::AcquireError),
    RecvError(tokio::sync::oneshot::error::RecvError),
    ParseInt(std::num::ParseIntError),
    TryFromSliceError(std::array::TryFromSliceError),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Tauri(tauri::Error),
    SerdeUrlEncodedSer(serde_urlencoded::ser::Error),
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

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Self::JoinError(e)
    }
}

impl From<tokio::sync::AcquireError> for Error {
    fn from(e: tokio::sync::AcquireError) -> Self {
        Self::AcquireError(e)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::RecvError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::TryFromSliceError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl From<tauri::Error> for Error {
    fn from(e: tauri::Error) -> Self {
        Self::Tauri(e)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(e: serde_urlencoded::ser::Error) -> Self {
        Self::SerdeUrlEncodedSer(e)
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

impl From<Vec<String>> for Error {
    fn from(e: Vec<String>) -> Self {
        Self::Other(e.join(", "))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Diesel(e) => write!(f, "Diesel error: {}", e),
            Self::DieselConnection(e) => write!(f, "Diesel connection error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::JoinError(e) => write!(f, "Join error: {}", e),
            Self::AcquireError(e) => write!(f, "Acquire error: {}", e),
            Self::RecvError(e) => write!(f, "Receive error: {}", e),
            Self::ParseInt(e) => write!(f, "Parse int error: {}", e),
            Self::TryFromSliceError(e) => write!(f, "Try from slice error: {}", e),
            Self::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Self::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
            Self::Tauri(e) => write!(f, "Tauri error: {}", e),
            Self::SerdeUrlEncodedSer(e) => {
                write!(f, "Serde URL encoded serialization error: {}", e)
            }
            Self::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        e.to_string()
    }
}
