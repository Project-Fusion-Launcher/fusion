use crate::common::result::Result;
use std::path::Path;
use tokio::fs;

#[cfg(target_os = "windows")]
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

    if !output_dir.exists() {
        fs::create_dir_all(output_dir).await?;
    }

    let exe_path = std::env::current_exe()?;
    let seven_zip = exe_path.parent().unwrap().join("thirdparty/7-Zip/7z.exe");

    let mut command = tokio::process::Command::new(seven_zip);
    command
        .arg("x")
        .arg(file_path)
        .arg(format!("-o{}", output_dir.to_string_lossy()))
        .arg("-aoa")
        .arg("-x!$PLUGINSDIR/*");

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(NO_WINDOW_FLAG);
    }

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

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(NO_WINDOW_FLAG);
    }

    let result = command.spawn().map_err(|e| e.to_string())?;

    println!("{:?}", result);

    Ok(())
}
