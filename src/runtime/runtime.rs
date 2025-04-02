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
    job: Arc<dyn Job<T, U> + Send + Sync>,
    work: Arc<Mutex<Work<T>>>,
    output: Sender<Batch<U>>,
}

//Update the worker status enum and Batch type to be one thing.
//Workers will have a Work::Awaiting or Work::Working(Batch<T>) 
//When workers have a Work::Awaiting, they will do nothing
//When workers have a Work::Working(Batch<T>), they will work on
//the batch until it is complete. Then they will set their own status to
//Work::Awaiting

#[derive(PartialEq)]
pub enum Work<T> {
    Awaiting,
    Delegated(Batch<T>),
    Completed,
}

pub struct Batcher<T> {
    data: Vec<T>,
}

#[derive(PartialEq)]
pub struct Batch<T> {
    pub id: usize,
    pub items: Vec<T>,
}

#[derive(PartialEq)]
pub enum DispatchResult<T> {
    Dispatched,
    AllWorkersBusy(Batch<T>),
}

// change plan - refactor batch so that cloning is not necessary,
// prefer to pass references and pass back data

impl<T> Batch<T> 
where
    T: Clone,
{
    pub fn new(id: usize) -> Self {
        let items = Vec::new();
        Self { id, items }
    }

    pub fn create_from_slice(id: usize, slice: &[T]) -> Self {
        let mut items = Vec::with_capacity(slice.len());
        items.extend_from_slice(slice);
        Self { id, items }
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
            job: Arc::clone(job),
            work: Arc::new(Mutex::new(Work::Awaiting)),
            output: outgoing.clone(),
        }
    }

    pub fn needs_work(&self) -> bool {
        match *self.work.lock().unwrap() {
            Work::Awaiting => true,
            _ => false,
        }
    }

    pub fn process_batch(&self, input_batch: &Batch<T>) -> Batch<U> {
        let mut output_batch = Batch::new(input_batch.id);
        for item in &input_batch.items {
            let output_item = self.job.run(&item); 
            output_batch.items.push(output_item);
        }
        output_batch
    }

    pub fn run(&self) {
        loop {
            let mut work = self.work.lock().unwrap();
            match &*work {
                Work::Delegated(batch) => {
                    let result = self.process_batch(batch);
                    let _ = self.output.send(result);
                    *work = Work::Awaiting
                }
                Work::Completed => return,
                Work::Awaiting => {}
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
            let mut work = worker.work.lock().unwrap();
            match &*work {
                Work::Awaiting => {
                    *work = Work::Delegated(batch);
                    return DispatchResult::Dispatched;
                }
                _ => {}
            }
        }
        return DispatchResult::AllWorkersBusy(batch);
    }

    pub fn execute(&self, data: Vec<T>, batch_count: usize) {
        for worker in &self.workers {
            let worker = Arc::clone(&worker);
            thread::spawn(move || {
                worker.run();
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
            let mut work = worker.work.lock().unwrap();
            *work = Work::Completed;
        }
        result.sort_by_key(|b| b.id);
        result.into_iter().flat_map(|b| b.items.into_iter()).collect()
    }
}
