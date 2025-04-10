use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

use super::batches::{WorkBatch, OutputBatch, Batcher};
use super::workpool::WorkPool;

use std::marker::PhantomData;

pub trait Job<T, U> {
    fn run(&self, input: &T) -> U;
}

pub trait WorkQueue<T> {
    fn new(id: usize, batches: Vec<WorkBatch<T>>) -> Self;

    fn empty(&self) -> bool;

    fn push(&mut self, batch: WorkBatch<T>);

    fn pop(&mut self) -> Option<WorkBatch<T>>;
}

pub trait WorkConfig: Send + Sync + 'static {
    const THREAD_COUNT: usize;
    const BATCH_COUNT: usize;

    type Input: Send + Sync + 'static;
    type Output: Send + Sync + 'static;
    type Job: Job<Self::Input, Self::Output> + Send + Sync + 'static; 
    type Queue: WorkQueue<Self::Input> + Send + Sync + 'static; 
}

pub struct Manager<WC: WorkConfig> {
    workers: Vec<Arc<Worker<WC>>>,
    collector: Receiver<OutputBatch<WC::Output>>,
}

impl<WC> Manager<WC>
where
    WC: WorkConfig,
{
    pub fn new(
        job: &Arc<WC::Job>,
        data: Vec<WC::Input>,
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(WC::THREAD_COUNT);
        let batches = Batcher::new(data).create_batches(WC::BATCH_COUNT);
        let work_pool = WorkPool::new(WC::THREAD_COUNT, batches);
        for (idx, queue) in work_pool.pool.iter().enumerate() {
            let worker = Arc::new(Worker::new(
                idx, job, &queue, &worker_tx
            ));
            workers.push(worker);
        }
        Self {
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

    pub fn join(&self) -> Vec<WC::Output> {
        let mut result = Vec::with_capacity(WC::BATCH_COUNT);
        for _ in 0..WC::BATCH_COUNT {
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

struct Worker<WC: WorkConfig> {
    _id: usize,
    job: Arc<WC::Job>,
    status: Arc<RwLock<WorkStatus>>,
    work: Arc<RwLock<WC::Queue>>,
    output: Sender<OutputBatch<WC::Output>>,
    _phantom: PhantomData<WC::Input>,
}

impl<WC> Worker<WC>
where
    WC: WorkConfig,
{
    pub fn new(
        _id: usize,
        job: &Arc<WC::Job>,
        work_queue: &Arc<RwLock<WC::Queue>>,
        outgoing: &Sender<OutputBatch<WC::Output>>,
    ) -> Self {
        Self {
            _id: _id,
            job: Arc::clone(job),
            status: Arc::new(RwLock::new(WorkStatus::Working)),
            work: Arc::clone(work_queue),
            output: outgoing.clone(),
            _phantom: PhantomData,
        }
    }

    pub fn get_work(&self) -> Option<WorkBatch<WC::Input>> {
        self.work.write().unwrap().pop()
    }

    pub fn process_batch(
        &self, input_batch: &WorkBatch<WC::Input>
    ) -> OutputBatch<WC::Output> {
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
