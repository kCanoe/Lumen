use std::sync::{Arc, RwLock};
use std::collections::VecDeque;

use super::batches::Batch;

pub struct WorkQueue<T> {
    pub id: usize,
    pub items: usize,
    work: VecDeque<Batch<T>>,
}

impl<T> WorkQueue<T> {
    pub fn empty() -> Self {
        Self {
            id: 999,
            items: 0,
            work: VecDeque::new(),
        }
    }

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
    pub pool: Vec<Arc<RwLock<WorkQueue<T>>>>,
}

impl<T> WorkPool<T> {
    pub fn new(worker_count: usize, mut batches: VecDeque<Batch<T>>) -> Self {
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

    pub fn has_work(&self) -> bool {
        let mut has_work = false;
        for queue in &self.pool {
            let queue = queue.read().unwrap();
            if queue.items > 0 {
                has_work = true;
                break;
            }
        }
        has_work
    }
}

