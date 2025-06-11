use anyhow::Result;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::task::JoinHandle;
use tokio_mpmc::Queue;

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

struct Job<T: Send + 'static> {
    task: Box<dyn FnOnce() -> BoxedFuture<T> + Send + 'static>,
}

pub struct WorkerPool<T: Send + 'static> {
    workers: Vec<JoinHandle<()>>,
    queue: Arc<Queue<Job<T>>>,
}

impl<T: Send + 'static> WorkerPool<T> {
    pub fn new(max_concurrency: usize) -> Self {
        let queue: Arc<Queue<Job<T>>> = Arc::new(Queue::new(max_concurrency * 2));

        let mut workers = Vec::with_capacity(max_concurrency);

        for _ in 0..max_concurrency {
            let queue = Arc::clone(&queue);

            let worker = tokio::spawn(async move {
                loop {
                    match queue.receive().await {
                        Ok(Some(job)) => {
                            let task = job.task;
                            task().await;
                        }
                        Ok(None) => break,
                        Err(e) => eprintln!("Receive failed: {}", e),
                    }
                }
            });

            workers.push(worker);
        }

        WorkerPool { workers, queue }
    }

    pub async fn execute<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        let job = Job {
            task: Box::new(move || Box::pin(f())),
        };

        self.queue.send(job).await?;
        Ok(())
    }

    pub async fn shutdown(self) {
        self.queue.close();

        for worker in self.workers {
            worker.await.unwrap();
        }
    }
}
