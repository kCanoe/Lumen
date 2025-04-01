use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

trait Operation<T, U> {
    fn run(&self, input: T) -> U;
}

pub struct Runtime<T, U> {
    workers: Vec<Arc<Worker<T, U>>>,
    handles: Vec<Arc<Mutex<Batch<T>>>>,
    collector: Receiver<U>,
}

pub struct Worker<T, U> {
    status: Arc<Mutex<WorkerStatus>>,
    job: Arc<dyn Operation<T, U> + Send + Sync>,
    input: Arc<Mutex<Batch<T>>>,    
    output: Sender<U>,
}

#[derive(Clone, Copy)]
pub enum WorkerStatus {
    Busy,
    Free,
}

pub struct Batcher<T> {
    data: Vec<T>
}

pub struct Batch<T> {
    id: usize,
    data: Vec<T>,
}

impl<T> Batch<T> 
where
    T: Clone,
{
    pub fn new(id: usize) -> Self {
        let data = Vec::new();
        Self { id, data }
    }

    pub fn create_from_slice(id: usize, slice: &[T]) -> Self {
        let mut data = Vec::with_capacity(slice.len());
        data.extend_from_slice(slice);
        Self { id, data }
    }
}

impl<T> Batcher<T>
where
    T: Clone,
{
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn create_batches(self, batch_count: usize) -> Vec<Batch<T>> {
        assert!(self.data.len() % batch_count == 0);
        let batch_size = self.data.len() / batch_count;
        let mut result = Vec::new();
        for i in 0..batch_count {
            let batch_slice = &self.data[batch_count * i..(batch_count+1)*i];
            let batch = Batch::create_from_slice(i, batch_slice);
            result.push(batch);
        }
        result
    }
}

impl<T, U> Worker<T, U>
where
    T: Clone + Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    pub fn new(
        job: &Arc<dyn Operation<T, U> + Send + Sync>,
        outgoing: &Sender<U>
    ) -> Self {
        Self {
            status: Arc::new(Mutex::new(WorkerStatus::Free)),
            job: Arc::clone(job),
            input: Arc::new(Mutex::new(Batch::new(0))),
            output: outgoing.clone(),
        }
    }

    pub fn handle(&self) -> Arc<Mutex<Batch<T>>> {
        Arc::clone(&self.input)
    }

    pub fn status(&self) -> WorkerStatus {
        let status = self.status.lock().unwrap();
        *status
    }

    pub fn start(&self) {
        todo!();
    }
}

impl<T, U> Runtime<T, U>
where
    T: Clone + Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    pub fn new(
        thread_count: usize,
        job: Arc<dyn Operation<T, U> + Send + Sync>
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(thread_count);
        let mut handles = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let worker = Arc::new(Worker::new(&job, &worker_tx));
            handles.push(worker.handle());
            workers.push(worker);
        }
        Self {
            workers,
            handles,
            collector,
        }
    }

    pub fn execute(&self, data: Vec<T>, batches: usize) {
        for worker in &self.workers {
            let worker = worker.clone();
            thread::spawn(move || {
                worker.start();
            });
        }
        let mut batches = Batcher::new(data)
            .create_batches(batches)
            .into_iter();
        while let Some(batch) = batches.next() {
            for i in 0..self.workers.len() {
                match self.workers[i].status() {
                    WorkerStatus::Free => {
                        let mut worker_batch = self.workers[i].input
                            .lock()
                            .unwrap();
                        *worker_batch = batch;
                        break;
                    }
                    WorkerStatus::Busy => {}
                }
            }
        }
    }

    pub fn join(&self) -> Vec<U> {
        let results = self.collector.recv();
        todo!();
    }
}


