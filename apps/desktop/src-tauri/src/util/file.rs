use crate::{common::result::Result, APP};
use std::path::Path;
#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt};
use tauri::{path::BaseDirectory, Manager};
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};

const SKIP_EXTENSIONS: &[&str] = &["dylib", "bundle", "so", "dll"];

#[cfg(windows)]
const NO_WINDOW_FLAG: u32 = 0x08000000;

#[cfg(target_os = "macos")]
const MACOS_MAGICS: &[[u8; 4]] = &[
    [0xCA, 0xFE, 0xBA, 0xBE],
    [0xCF, 0xFA, 0xED, 0xFE],
    [0xCE, 0xFA, 0xED, 0xFE],
];

#[cfg(target_os = "linux")]
const LINUX_MAGICS: &[[u8; 4]] = &[[0x7f, 0x45, 0x4c, 0x46]];

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

    #[cfg(target_os = "windows")]
    let os_specific_path = "thirdparty/7-Zip/windows/7z.exe";
    #[cfg(target_os = "linux")]
    let os_specific_path = "thirdparty/7-Zip/linux/7zzs";
    #[cfg(target_os = "macos")]
    let os_specific_path = "thirdparty/7-Zip/macos/7zz";

    let seven_zip = APP
        .get()
        .unwrap()
        .path()
        .resolve(os_specific_path, BaseDirectory::Resource)?;

    let mut command = tokio::process::Command::new(seven_zip);
    command
        .arg("x")
        .arg(file_path)
        .arg(format!("-o{}", output_dir.to_string_lossy()))
        .arg("-aoa")
        .arg("-x!$PLUGINSDIR/*");

    #[cfg(windows)]
    command.creation_flags(NO_WINDOW_FLAG);

    let result = command.output().await?;

    println!("7z output: {:?}", String::from_utf8_lossy(&result.stdout));

    if !result.status.success() {
        return Err(format!("Failed to extract file: {:?}", file_path).into());
    }

    fs::remove_file(file_path).await?;

    Ok(())
}

pub fn execute_file<P>(file_path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();

    if !file_path.exists() {
        return Err(format!("File does not exist: {:?}", file_path).into());
    }

    let mut command = tokio::process::Command::new(file_path);

    let parent_dir = file_path.parent().unwrap();
    command.current_dir(parent_dir);

    #[cfg(windows)]
    command.creation_flags(NO_WINDOW_FLAG);

    let result = command.spawn().map_err(|e| e.to_string())?;

    println!("{:?}", result);

    Ok(())
}

pub async fn is_executable(file_path: &Path) -> bool {
    if !file_path
        .extension()
        .and_then(|e| Some(!SKIP_EXTENSIONS.contains(&e.to_str()?.to_lowercase().as_str())))
        .unwrap_or(true)
    {
        return false;
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(mut file) = File::open(file_path).await {
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
        if let Ok(mut file) = File::open(file_path).await {
            let mut magic = [0u8; 4];
            if file.read_exact(&mut magic).await.is_ok() {
                return MACOS_MAGICS.contains(&magic);
            };
        }
    }

    false
}

#[cfg(unix)]
pub async fn set_permissions<P: AsRef<Path>>(file_path: P, mode: u32) -> Result<()> {
    fs::set_permissions(file_path, Permissions::from_mode(mode)).await?;

    Ok(())
}
