use super::result::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct WorkerPool {
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
        }
    }

    pub async fn execute_task<F>(&self, task_future: F) -> Result<()>
    where
        F: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        let permit = self.semaphore.clone().acquire_owned().await?;
        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = task_future.await {
                println!("Worker encountered an error: {:?}", e);
            }
        });
        Ok(())
    }
}
