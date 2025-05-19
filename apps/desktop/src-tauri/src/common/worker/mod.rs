use super::result::Result;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::{sync::mpsc, task::JoinHandle};

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

struct Job<T: Send + 'static> {
    task: Box<dyn FnOnce() -> BoxedFuture<T> + Send + 'static>,
}

pub struct WorkerPool<T: Send + 'static> {
    sender: mpsc::Sender<Job<T>>,
    workers: Vec<JoinHandle<()>>,
}

impl<T: Send + 'static> WorkerPool<T> {
    pub fn new(max_concurrency: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job<T>>(1024);
        let receiver = Arc::new(tokio::sync::Mutex::new(receiver));

        let mut workers = Vec::with_capacity(max_concurrency);

        for _ in 0..max_concurrency {
            let receiver = receiver.clone();

            let worker = tokio::spawn(async move {
                loop {
                    let job = {
                        let mut receiver = receiver.lock().await;
                        receiver.recv().await
                    };

                    if let Some(job) = job {
                        let task = job.task;
                        task().await;
                    } else {
                        break;
                    }
                }
            });

            workers.push(worker);
        }

        WorkerPool { sender, workers }
    }

    pub async fn execute<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        let job = Job {
            task: Box::new(move || Box::pin(f())),
        };

        self.sender.send(job).await.map_err(|e| {
            crate::common::result::Error::from(format!("WorkerPool send error: {}", e))
        })?;
        Ok(())
    }

    pub async fn shutdown(self) {
        drop(self.sender);

        for worker in self.workers {
            worker.await.unwrap();
        }
    }
}
