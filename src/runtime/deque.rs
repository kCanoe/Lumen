// a ring buffer that will be the underlying data struture for the deque
struct RingBuffer<T, const C: usize> {
    buff: [T; C],
    capacity: usize,
    mask: usize,
}

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

pub struct Deque<T, const C: usize> {
    top: usize,
    bottom: usize,
    buffer: RingBuffer<T, C>,
}

impl<T, const C: usize> Deque<T, C>
where
    T: Sized + Default + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            top: 0,
            bottom: 0,
            buffer: RingBuffer::<T, C>::new(),
        }
    }
}





