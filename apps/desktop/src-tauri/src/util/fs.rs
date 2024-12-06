use crate::common::result::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

const BLACKLISTED_LAUNCH_TARGETS: [&str; 6] = [
    "unitycrashhandler64.exe",
    "unitycrashhandler32.exe",
    "ue4prereqsetup_x64.exe",
    "ueprereqsetup_x64.exe",
    "dxwebsetup.exe",
    "uninstall.exe",
];

const NO_WINDOW_FLAGS: u32 = 0x08000000;

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
            if !BLACKLISTED_LAUNCH_TARGETS.contains(
                &entry_path
                    .file_name()
                    .unwrap()
                    .to_ascii_lowercase()
                    .to_str()
                    .unwrap(),
            ) {
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

pub async fn extract_file<P>(file_path: &P, output_dir: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let output_dir = output_dir.as_ref();

    if !file_path.exists() {
        return Err(format!("File does not exist: {:?}", file_path).into());
    }

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).await?;
    }

    let exe_path = std::env::current_exe()?;
    let seven_zip = exe_path.parent().unwrap().join("thirdparty/7-Zip/7z.exe");

    let result = tokio::process::Command::new(seven_zip)
        .arg("x")
        .arg(file_path)
        .arg(format!("-o{}", output_dir.to_string_lossy()))
        .arg("-aoa")
        .arg("-x!$PLUGINSDIR/*")
        .creation_flags(NO_WINDOW_FLAGS)
        .output()
        .await?;

    println!("7z output: {:?}", String::from_utf8_lossy(&result.stdout));

    if !result.status.success() {
        return Err(format!("Failed to extract file: {:?}", file_path).into());
    }

    fs::remove_file(file_path).await?;

    Ok(())
}
