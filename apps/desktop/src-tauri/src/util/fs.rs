use crate::common::error::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

const BLACKLISTED_LAUNCH_TARGETS: [&str; 5] = [
    "UnityCrashHandler64.exe",
    "UnityCrashHandler32.exe",
    "UE4PrereqSetup_x64.exe",
    "UEPrereqSetup_x64.exe",
    "dxwebsetup.exe",
];

/// Given a path, finds an appropriate launch target (e.g. an executable) to run.
/// It may not return the proper launch target if there are multiple executables.
pub async fn find_launch_target<P>(dir: P) -> Result<Option<PathBuf>>
where
    P: AsRef<Path>,
{
    let path = dir.as_ref();
    if !path.exists() || !path.is_dir() {
        return Ok(None);
    }

    let mut entries = fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();

        if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "exe") {
            if !BLACKLISTED_LAUNCH_TARGETS
                .contains(&entry_path.file_name().unwrap().to_str().unwrap())
            {
                return Ok(Some(entry_path));
            }
        } else if entry_path.is_dir() {
            if let Some(target) = Box::pin(find_launch_target(entry_path)).await? {
                return Ok(Some(target));
            }
        }
    }

    Ok(None)
}
