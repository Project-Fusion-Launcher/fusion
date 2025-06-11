use crate::APP_ID;
use std::path::PathBuf;

pub struct PathResolver;

impl PathResolver {
    pub fn data_dir() -> PathBuf {
        dirs::data_dir().unwrap()
    }

    pub fn app_data_dir() -> PathBuf {
        Self::data_dir().join(APP_ID)
    }
}
