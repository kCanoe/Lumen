use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::VecDeque;

pub trait Job<T, U> {
    fn run(&self, input: &T) -> U;
}

pub struct Manager<T, U> {
    workers: Vec<Arc<Worker<T, U>>>,
    collector: Receiver<Batch<U>>,
}

pub struct Worker<T, U> {
    status: Arc<Mutex<WorkerStatus>>,
    job: Arc<dyn Job<T, U> + Send + Sync>,
    input: Arc<Mutex<Batch<T>>>,
    output: Sender<Batch<U>>,
}

//Update the worker status enum and Batch type to be one thing.
//Workers will have a Work::Awaiting or Work::Working(Batch<T>) 
//When workers have a Work::Awaiting, they will do nothing
//When workers have a Work::Working(Batch<T>), they will work on
//the batch until it is complete. Then they will set their own status to
//Work::Awaiting


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WorkerStatus {
    HasWork,
    Free,
    Done,
}

pub struct Batcher<T> {
    data: Vec<T>,
}

#[derive(PartialEq)]
pub struct Batch<T> {
    pub id: usize,
    pub data: Vec<T>,
}

#[derive(PartialEq)]
pub enum DispatchResult<T> {
    Dispatched,
    AllWorkersBusy(Batch<T>),
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

    pub fn create_batches(self, batch_count: usize) -> VecDeque<Batch<T>> {
        assert!(self.data.len() % batch_count == 0);
        let batch_size = self.data.len() / batch_count;
        let mut result = VecDeque::new();
        for i in 0..batch_count {
            let batch_slice = &self.data[batch_size * i..batch_size * (i + 1)];
            let batch = Batch::create_from_slice(i, batch_slice);
            result.push_back(batch);
        }
        result
    }
}

impl<T, U> Worker<T, U>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
{
    pub fn new(
        job: &Arc<dyn Job<T, U> + Send + Sync>,
        outgoing: &Sender<Batch<U>>,
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
        loop {
            match self.status() {
                WorkerStatus::Free => {}
                WorkerStatus::HasWork => {
                    let items = self.input.lock().unwrap();
                    let mut result = Batch::new(items.id);
                    for item in &items.data {
                        let output = self.job.run(item);
                        result.data.push(output);
                    }
                    let _ = self.output.send(result);
                    let mut status = self.status.lock().unwrap();
                    *status = WorkerStatus::Free;
                }
                WorkerStatus::Done => return,
            }
        }
    }
}

impl<T, U> Manager<T, U>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
{
    pub fn new(
        thread_count: usize,
        job: Arc<dyn Job<T, U> + Send + Sync>,
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            let worker = Arc::new(Worker::new(&job, &worker_tx));
            workers.push(worker);
        }
        Self { workers, collector }
    }

    pub fn try_dispatch(&self, batch: Batch<T>) -> DispatchResult<T> {
        for worker in &self.workers {
            if worker.status() == WorkerStatus::Free {
                let mut worker_batch = worker.input.lock().unwrap();
                *worker_batch = batch;
                let mut status = worker.status.lock().unwrap();
                *status = WorkerStatus::HasWork;
                return DispatchResult::Dispatched;
            }
        }
        return DispatchResult::AllWorkersBusy(batch);
    }

    pub fn execute(&self, data: Vec<T>, batch_count: usize) {
        for worker in &self.workers {
            let worker = Arc::clone(&worker);
            thread::spawn(move || {
                worker.start();
            });
        }
        let mut batches = Batcher::new(data).create_batches(batch_count);
        while let Some(batch) = batches.pop_front() {
            match self.try_dispatch(batch) {
                DispatchResult::Dispatched => {}
                DispatchResult::AllWorkersBusy(batch) => {
                    batches.push_back(batch);
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            } 
        }
    }

    pub fn join(&self, batch_count: usize) -> Vec<U> {
        let mut result: Vec<Batch<U>> = Vec::with_capacity(batch_count);
        for _ in 0..batch_count {
            let output_batch = self.collector.recv().unwrap();
            result.push(output_batch);
        }
        for worker in &self.workers {
            let mut status = worker.status.lock().unwrap();
            *status = WorkerStatus::Done;
        }
        result.sort_by_key(|b| b.id);
        result.into_iter().flat_map(|b| b.data.into_iter()).collect()
    }
}
