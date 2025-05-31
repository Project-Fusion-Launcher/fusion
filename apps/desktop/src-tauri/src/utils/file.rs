use crate::{common::result::Result, APP};
use async_trait::async_trait;
use std::path::Path;
use tauri::{path::BaseDirectory, Manager};
use tokio::{
    fs::{self, File, OpenOptions},
    io,
    process::Command,
};

#[cfg(windows)]
const NO_WINDOW_FLAG: u32 = 0x08000000;

pub async fn extract_file<P>(file_path: &P, output_dir: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let output_dir = output_dir.as_ref();

    if !file_path.exists() {
        return Err(format!("File does not exist: {:?}", file_path).into());
    }

    fs::create_dir_all(output_dir).await?;

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

    let mut command = Command::new(seven_zip);
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

    let mut command = Command::new(file_path);

    let parent_dir = file_path.parent().unwrap();
    command.current_dir(parent_dir);

    #[cfg(windows)]
    command.creation_flags(NO_WINDOW_FLAG);

    let result = command.spawn().map_err(|e| e.to_string())?;

    println!("{:?}", result);

    Ok(())
}

#[cfg(unix)]
pub async fn set_permissions<P: AsRef<Path>>(file_path: P, mode: u32) -> Result<()> {
    use std::{fs::Permissions, os::unix::fs::PermissionsExt};

    fs::set_permissions(file_path, Permissions::from_mode(mode)).await?;

    Ok(())
}

#[async_trait]
pub trait OpenWithDirs {
    async fn open_with_dirs<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<File>;
}

#[async_trait]
impl OpenWithDirs for OpenOptions {
    async fn open_with_dirs<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<File> {
        let path_ref = path.as_ref();
        if let Some(parent) = path_ref.parent() {
            fs::create_dir_all(parent).await?;
        }
        self.open(path_ref).await
    }
}
