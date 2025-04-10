use std::sync::atomic::AtomicUsize;

// lint macro used to suppress warnings while working on other changes

#[allow(dead_code)]
struct RingBuffer<T, const C: usize> {
    buff: [T; C],
    capacity: usize,
    mask: usize,
}

#[allow(dead_code)]
impl<T, const C: usize> RingBuffer<T, C>
where
    T: Sized + Default + Clone + Copy,
{
    // capacity must be some n where n = 2^k 
    pub fn new() -> Self {
        Self {
            buff: [T::default(); C],
            capacity: C,
            mask: C - 1,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn store(&mut self, i: usize, val: T) {
        self.buff[i & self.mask] = val;
    }

    pub fn load(&self, i: usize) -> T {
        self.buff[i & self.mask]
    }
}

#[allow(dead_code)]
pub struct Deque<T, const C: usize> {
    top: AtomicUsize,
    bottom: AtomicUsize, 
    buffer: RingBuffer<T, C>,
}

impl<T, const C: usize> Deque<T, C>
where
    T: Sized + Default + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            top: AtomicUsize::new(0),
            bottom: AtomicUsize::new(0),
            buffer: RingBuffer::<T, C>::new(),
        }
    }
}





