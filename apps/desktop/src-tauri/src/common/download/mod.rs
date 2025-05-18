use crate::models::download::{Download, DownloadFile};
use tokio::fs::{self, OpenOptions};

use super::{result::Result, worker::WorkerPool};

pub async fn process_download(download: Download) -> Result<()> {
    let pool = WorkerPool::new(2);
    fs::create_dir_all(&download.path).await?;

    for file in download.files {
        println!("Processing file: {}", file.filename);
        pool.execute(move || process_file(file)).await?;
    }

    pool.shutdown().await;

    Ok(())
}

pub async fn process_file(download_file: DownloadFile) -> Result<()> {
    /*let mut file = OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open(&download_file.filename)
    .await?; */

    println!("Downloading file: {}", download_file.filename);

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    Ok(())
}
