use crate::common::result::Result;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};
use tokio::fs;

use super::file;

const BLACKLISTED_LAUNCH_TARGETS: [&str; 6] = [
    "unitycrashhandler64.exe",
    "unitycrashhandler32.exe",
    "ue4prereqsetup_x64.exe",
    "ueprereqsetup_x64.exe",
    "dxwebsetup.exe",
    "uninstall.exe",
];

/// Given a path, finds an appropriate launch target (e.g. an executable) to run.
/// It may not return the proper launch target if there are multiple executables.
pub async fn find_launch_target<P>(dir: P) -> Result<Option<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(dir.as_ref().to_path_buf());

    while let Some(current_dir) = queue.pop_front() {
        if let Ok(mut entries) = fs::read_dir(&current_dir).await {
            let mut subdirs = Vec::new();
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !BLACKLISTED_LAUNCH_TARGETS.contains(&name.to_lowercase().as_str())
                            && file::is_executable(&path).await
                        {
                            return Ok(Some(path));
                        }
                    }
                } else if path.is_dir() {
                    subdirs.push(path);
                }
            }
            queue.extend(subdirs);
        }
    }

    Ok(None)
}
