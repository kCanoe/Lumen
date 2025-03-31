use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;
use std::thread;

pub struct Runtime<T, U> {
    workers: Vec<Worker<T, U>>,
    handles: Vec<Sender<T>>, 
    collector: Receiver<U>,
}

trait Operation<T, U> {
    fn run(&self, input: T) -> U;
}

pub struct Worker<T, U> {
    job: Arc<dyn Operation<T, U>>,
    incoming: Receiver<T>,    
    outgoing: Sender<U>,
}

impl<T, U> Worker<T, U> {
    pub fn new(
        job: Arc<dyn Operation<T, U>>,
        outgoing: &Sender<U>,
    ) -> (Self, Sender<T>) {
        let (handle, incoming) = channel();
        let worker = Self {
            job,
            incoming,
            outgoing: outgoing.clone(),
        };
        (worker, handle)
    }
}

impl<T, U> Runtime<T, U> {
    pub fn new(threads: usize, job: Arc<dyn Operation<T, U>>) -> Self {
        let (worker_tx, collector) = channel();

        let mut workers = Vec::with_capacity(threads);
        let mut handles = Vec::with_capacity(threads);

        for _ in 0..threads {
            let job = job.clone();
            let (worker, handle) = Worker::new(job, &worker_tx);
            workers.push(worker);
            handles.push(handle);
        }

        Self {
            workers,
            handles,
            collector,
        }
    }
}
