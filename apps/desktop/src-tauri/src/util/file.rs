use crate::common::result::Result;
use std::path::Path;
use std::time::Instant;
use tokio::{
    fs,
    io::{self, AsyncReadExt},
};

const NO_WINDOW_FLAGS: u32 = 0x08000000;

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

pub async fn verify_md5<P>(file_path: &P, md5: &str) -> Result<bool>
where
    P: AsRef<Path>,
{
    let start = Instant::now();

    let file_path = file_path.as_ref();

    if !file_path.exists() {
        return Err(format!("File does not exist: {:?}", file_path).into());
    }

    let file = fs::File::open(file_path).await?;
    let mut reader = io::BufReader::new(file);

    let mut hasher = md5::Context::new();
    let mut buffer = [0; 16 * 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.consume(&buffer[..bytes_read]);
    }

    let digest = hasher.compute();

    let duration = start.elapsed();
    println!("MD5 verification took: {:?}", duration);
    println!("MD5: {:x}", digest);
    println!("Expected MD5: {}", md5);

    Ok(format!("{:x}", digest) == md5)
}
