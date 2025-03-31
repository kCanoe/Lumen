use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;
use std::thread;

trait Operation<T, U> {
    fn run(&self, input: T) -> U;
}

pub struct Runtime<T, U> {
    workers: Vec<Arc<Worker<T, U>>>,
    collector: Receiver<U>,
}

pub struct Worker<T, U> {
    status: WorkerStatus,
    job: Arc<dyn Operation<T, U> + Send + Sync>,
    input: Arc<T>,    
    output: Sender<U>,
}

pub enum WorkerStatus {
    Busy,
    Free,
}

impl<T, U> Worker<T, U>
where
    T: Default + Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    pub fn new(
        job: &Arc<dyn Operation<T, U> + Send + Sync>,
        outgoing: &Sender<U>
    ) -> Self {
        Self {
            status: WorkerStatus::Free,
            job: Arc::clone(job),
            input: Arc::new(T::default()),
            output: outgoing.clone(),
        }
    }

    pub fn status(&self) -> &WorkerStatus {
        &self.status
    }

    pub fn start(&self) {
        println!("worker started")
    }
}

impl<T, U> Runtime<T, U>
where
    T: Default + Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    pub fn new(
        thread_count: usize,
        job: Arc<dyn Operation<T, U> + Send + Sync>
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(thread_count);

        for _ in 0..thread_count {
            let worker = Arc::new(Worker::new(&job, &worker_tx));
            workers.push(worker);
        }

        Self {
            workers,
            collector,
        }
    }

    pub fn start(&self) {
        for worker in &self.workers {
            let worker = worker.clone();
            thread::spawn(move || {
                worker.start();
            });
        }
    }
}
