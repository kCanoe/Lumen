use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::collections::VecDeque;

use super::batches::{WorkBatch, OutputBatch, Batcher};
use super::workpool::{WorkQueue, WorkPool};

use std::marker::PhantomData;

pub trait Job<T, U> {
    fn run(&self, input: &T) -> U;
}

pub struct Manager<T, U, J, Q: WorkQueue<T>> {
    thread_count: usize,
    batch_count: usize,
    workers: Vec<Arc<Worker<T, U, J, Q>>>,
    work_pool: WorkPool<T, Q>,
    collector: Receiver<OutputBatch<U>>,
}

impl<T, U, J, Q> Manager<T, U, J, Q>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    J: Job<T, U> + Send + Sync + 'static,
    Q: WorkQueue<T> + Send + Sync + 'static,
{
    pub fn new(
        thread_count: usize,
        batch_count: usize,
        data: Vec<T>,
        job: Arc<J>,
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(thread_count);
        let mut batches = Batcher::new(data).create_batches(batch_count);
        let mut work_pool = WorkPool::new(thread_count, batches);
        for (idx, queue) in work_pool.pool.iter().enumerate() {
            let worker = Arc::new(Worker::new(idx, &job, &queue, &worker_tx));
            workers.push(worker);
        }
        Self {
            thread_count,
            batch_count,
            work_pool,
            workers,
            collector,
        }
    }

    pub fn execute(&self) {
        for worker in &self.workers {
            let worker = Arc::clone(&worker);
            thread::spawn(move || {
                worker.run();
            });
        }
    }

    pub fn join(&self) -> Vec<U> {
        let mut result = Vec::with_capacity(self.batch_count);
        for _ in 0..self.batch_count {
            let output_batch = self.collector.recv().unwrap();
            result.push(output_batch);
        }
        for worker in &self.workers {
            let mut status = worker.status.write().unwrap();
            *status = WorkStatus::Completed;
        }
        result.sort_by_key(|b| b.id);
        result.into_iter().flat_map(|b| b.into_iter()).collect()
    }
}

enum WorkStatus {
    Working,
    Completed,
}

struct Worker<T, U, J, Q> {
    id: usize,
    job: Arc<J>,
    status: Arc<RwLock<WorkStatus>>,
    work: Arc<RwLock<Q>>,
    output: Sender<OutputBatch<U>>,
    _phantom: PhantomData<T>,
}

impl<T, U, J, Q> Worker<T, U, J, Q>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    J: Job<T, U> + Send + Sync + 'static,
    Q: WorkQueue<T>,
{
    pub fn new(
        id: usize,
        job: &Arc<J>,
        work_queue: &Arc<RwLock<Q>>,
        outgoing: &Sender<OutputBatch<U>>,
    ) -> Self {
        Self {
            id: id,
            job: Arc::clone(job),
            status: Arc::new(RwLock::new(WorkStatus::Working)),
            work: Arc::clone(work_queue),
            output: outgoing.clone(),
            _phantom: PhantomData,
        }
    }

    pub fn get_work(&self) -> Option<WorkBatch<T>> {
        self.work.write().unwrap().pop()
    }

    pub fn process_batch(&self, input_batch: &WorkBatch<T>) -> OutputBatch<U> {
        let mut output_batch = OutputBatch::new(input_batch.id);
        for item in &input_batch.items {
            let output_item = self.job.run(&item); 
            output_batch.items.push(output_item);
        }
        output_batch
    }

    pub fn run(&self) {
        loop {
            if let Some(batch) = self.get_work() {
                let output_batch = self.process_batch(&batch);
                let _ = self.output.send(output_batch);
            }
        }
    }
}
