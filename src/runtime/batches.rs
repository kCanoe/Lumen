use std::collections::VecDeque;

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
