use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use std::collections::VecDeque;

pub trait Job<T, U> {
    fn run(&self, input: &T) -> U;
}

pub struct Manager<T, U, J> {
    workers: Vec<Arc<Worker<T, U, J>>>,
    collector: Receiver<Batch<U>>,
}

impl<T, U, J> Manager<T, U, J>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    J: Job<T, U> + Send + Sync + 'static,
{
    pub fn new(
        thread_count: usize,
        job: Arc<J>,
    ) -> Self {
        let (worker_tx, collector) = channel();
        let mut workers = Vec::with_capacity(thread_count);
        for i in 0..thread_count {
            let worker = Arc::new(Worker::new(i, &job, &worker_tx));
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
        result.into_iter().flat_map(|b| b.into_iter()).collect()
    }
}

pub struct Worker<T, U, J> {
    id: usize,
    job: Arc<J>,
    work: Arc<RwLock<WorkQueue<T>>>,
    output: Sender<Batch<U>>,
}

impl<T, U, J> Worker<T, U, J>
where
    T: Send + Sync + 'static,
    U: Send + Sync + 'static,
    J: Job<T, U> + Send + Sync + 'static,
{
    pub fn new(
        id: usize,
        job: &Arc<J>,
        outgoing: &Sender<Batch<U>>,
    ) -> Self {
        Self {
            id: id,
            job: Arc::clone(job),
            work: Arc::new(Mutex::new(Work::Awaiting)),
            output: outgoing.clone(),
        }
    }

    pub fn get_work(&self) -> Option<Batch<T>> {
        let work = self.work.write().unwrap();
        let batch = work.pop();
        batch
    }

    pub fn process_batch(&self, input_batch: &Batch<T>) -> Batch<U> {
        let mut output_batch = Batch::new(input_batch.id);
        for item in input_batch {
            let output_item = self.job.run(&item); 
            output_batch.items.push(output_item);
        }
        output_batch
    }

    pub fn run(&self) {
        loop {
            if let Some(batch) = self.get_work() {
                let output_batch = process_batch(&batch);
                output.send(output_batch);
            }
        }
    }
}

pub struct Batch<T> {
    pub id: usize,
    pub items: Vec<T>,
}

impl<T> Batch<T> {
    pub fn new(id: usize) -> Self {
        let items = Vec::new();
        Self { id, items }
    }

    pub fn from_vec(id: usize, items: Vec<T>) -> Self {
        Self { id, items }
    }
}

impl<T> IntoIterator for Batch<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Batch<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Batch<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}


pub struct Batcher<T> {
    items: Vec<T>,
}

impl<T> Batcher<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn create_batches(&mut self, batch_count: usize) -> VecDeque<Batch<T>> {
        assert!(self.items.len() % batch_count == 0);
        let batch_size = self.items.len() / batch_count;
        let mut work_batches = VecDeque::with_capacity(batch_count);
        for i in 0..batch_count {
            let batch: Vec<T> = self.items.drain(..batch_size).collect();
            let batch = Batch::from_vec(i, batch);
            work_batches.push_back(batch);
        }
        work_batches
    }
}

pub struct WorkQueue<T> {
    pub id: usize,
    pub items: usize,
    work: VecDeque<Batch<T>>,
}

impl<T> WorkQueue<T> {
    pub fn new(id: usize, batches: Vec<Batch<T>>) -> Self {
        Self {
            id: id,
            items: batches.len(),
            work: VecDeque::from(batches),
        }
    }
    
    pub fn push(&mut self, batch: Batch<T>) {
        self.work.push_front(batch);
    }

    pub fn pop(&mut self) -> Option<Batch<T>> {
        self.work.pop_back()
    }

    pub fn steal(&mut self) -> Option<Batch<T>> {
        self.work.pop_front()
    }
}

pub struct WorkPool<T> {
    pool: Vec<Arc<RwLock<WorkQueue<T>>>>,
}

impl<T> WorkPool<T> {
    pub fn new(worker_count: usize, batches: &mut VecDeque<Batch<T>>) -> Self {
        let batch_count = batches.len() / worker_count;
        let mut pool = Vec::with_capacity(worker_count);
        for i in 0..worker_count {
            let queue_batches = batches.drain(..batch_count).collect();
            let queue = Arc::new(RwLock::new(WorkQueue::new(i, queue_batches)));
            pool.push(queue);
        }
        Self { pool }
    }

    pub fn rebalance(&mut self, least: usize, most: usize) {
        let mut a = self.pool[most].write().unwrap();
        let mut b = self.pool[most].write().unwrap();
        let half_diff = (a.items - b.items) / 2;
        for _ in 0..half_diff {
            if let Some(batch) = a.steal() {
                b.push(batch);
            }
        }
    }

    pub fn smallest_pool(&self) -> usize {
        let mut least_idx = 0;
        for (idx, queue) in self.pool.iter().enumerate() {
            let current = queue.read().unwrap();
            let current_items = current.items;
            let least_lock = &self.pool[least_idx].read().unwrap();
            let least_items = least_lock.items;
            if current_items < least_items {
                least_idx = idx;
            }
        }
        least_idx
    }

    pub fn largest_pool(&self) -> usize {
        let mut greatest_idx = 0;
        for (idx, queue) in self.pool.iter().enumerate() {
            let current = queue.read().unwrap();
            let current_items = current.items;
            let greatest_lock = &self.pool[greatest_idx].read().unwrap();
            let greatest_items = greatest_lock.items;
            if current_items > greatest_items {
                greatest_idx = idx;
            }
        }
        greatest_idx
    }
}

