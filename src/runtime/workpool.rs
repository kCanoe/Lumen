use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use std::marker::PhantomData;

use super::batches::WorkBatch;
use super::WorkQueue;

pub struct BaseQueue<T> {
    pub id: usize,
    pub items: usize,
    work: VecDeque<WorkBatch<T>>,
}

impl<T> WorkQueue<T> for BaseQueue<T> {
    fn new(id: usize, batches: Vec<WorkBatch<T>>) -> Self {
        Self {
            id: id,
            items: batches.len(),
            work: VecDeque::from(batches),
        }
    }

    fn empty(&self) -> bool {
        self.items <= 0
    }
    
    fn push(&mut self, batch: WorkBatch<T>) {
        self.work.push_front(batch);
    }

    fn pop(&mut self) -> Option<WorkBatch<T>> {
        self.work.pop_back()
    }
}

pub struct WorkPool<T, Q> {
    pub pool: Vec<Arc<RwLock<Q>>>,
    _phantom: PhantomData<T>,
}

impl<T, Q> WorkPool<T, Q>
where
    Q: WorkQueue<T>,
{
    pub fn new(
        worker_count: usize,
        mut batches: VecDeque<WorkBatch<T>>
    ) -> Self {
        let batch_count = batches.len() / worker_count;
        let mut pool = Vec::with_capacity(worker_count);
        for i in 0..worker_count {
            let queue_batches = batches.drain(..batch_count).collect();
            let queue = Arc::new(RwLock::new(Q::new(i, queue_batches)));
            pool.push(queue);
        }
        Self { pool, _phantom: PhantomData, }
    }
}
