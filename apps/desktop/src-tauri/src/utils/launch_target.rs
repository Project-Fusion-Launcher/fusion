use crate::common::result::Result;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};
use tokio::{fs, io::AsyncReadExt};

const BLACKLISTED_LAUNCH_TARGETS: [&str; 6] = [
    "unitycrashhandler64.exe",
    "unitycrashhandler32.exe",
    "ue4prereqsetup_x64.exe",
    "ueprereqsetup_x64.exe",
    "dxwebsetup.exe",
    "uninstall.exe",
];

const IGNORE_EXTENSIONS: &[&str] = &["dylib", "bundle", "so", "dll"];

#[cfg(target_os = "macos")]
const MACOS_MAGICS: &[[u8; 4]] = &[
    [0xCA, 0xFE, 0xBA, 0xBE],
    [0xCF, 0xFA, 0xED, 0xFE],
    [0xCE, 0xFA, 0xED, 0xFE],
];

#[cfg(target_os = "linux")]
const LINUX_MAGICS: &[[u8; 4]] = &[[0x7f, 0x45, 0x4c, 0x46]];

/// Given a path, finds an appropriate launch target (e.g. an executable) to run.
/// It may not return the proper launch target if there are multiple executables.
pub async fn find_launch_target<P>(dir: P) -> Result<Option<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut queue: VecDeque<PathBuf> = VecDeque::new();
    queue.push_back(dir.as_ref().to_path_buf());

    while let Some(current_dir) = queue.pop_front() {
        #[cfg(target_os = "macos")]
        if current_dir.to_string_lossy().ends_with(".app") {
            if let Ok(result) = find_launch_target_in_macos_app(&current_dir).await {
                return Ok(Some(result));
            }
        }
        if let Ok(mut entries) = fs::read_dir(&current_dir).await {
            let mut subdirs = Vec::new();
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !BLACKLISTED_LAUNCH_TARGETS.contains(&name.to_lowercase().as_str())
                            && is_executable(&path).await
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

#[cfg(target_os = "macos")]
async fn find_launch_target_in_macos_app<P: AsRef<Path>>(app_dir: P) -> Result<PathBuf> {
    let app_dir = app_dir.as_ref();
    let plist_path = app_dir.join("Contents").join("Info.plist");

    if !plist_path.exists() {
        return Err("Plist file not found".into());
    }

    let plist = plist::Value::from_file(&plist_path).map_err(|e| e.to_string())?;

    let target = plist.as_dictionary().and_then(|dict| {
        dict.get("CFBundleExecutable")
            .and_then(|value| value.as_string())
    });

    if let Some(target) = target {
        let target_path = app_dir.join("Contents").join("MacOS").join(target);
        if target_path.exists() {
            return Ok(target_path);
        }
    }
    Err("Launch target not found".into())
}

async fn is_executable(file_path: &Path) -> bool {
    if !file_path
        .extension()
        .and_then(|e| Some(!IGNORE_EXTENSIONS.contains(&e.to_str()?.to_lowercase().as_str())))
        .unwrap_or(true)
    {
        return false;
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(mut file) = fs::File::open(file_path).await {
            let mut magic = [0u8; 4];
            if file.read_exact(&mut magic).await.is_ok() {
                return LINUX_MAGICS.contains(&magic);
            };
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            return ["exe", "bat", "cmd", "com", "ps1"].contains(&ext.to_lowercase().as_str());
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(mut file) = fs::File::open(file_path).await {
            let mut magic = [0u8; 4];
            if file.read_exact(&mut magic).await.is_ok() {
                return MACOS_MAGICS.contains(&magic);
            };
        }
    }

    false
}
