use crate::models::download::{Download, DownloadFile};
use tokio::fs::{self, OpenOptions};

use super::{result::Result, worker::WorkerPool};

pub async fn process_download(download: Download) -> Result<()> {
    let pool = WorkerPool::new(5);
    fs::create_dir_all(&download.path).await?;

    for file in download.files {
        pool.execute_task(process_file(file)).await?;
    }

    Ok(())
}

pub async fn process_file(download_file: DownloadFile) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&download_file.filename)
        .await?;

    Ok(())
}
