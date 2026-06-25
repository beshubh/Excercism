use std::collections::VecDeque;


pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    capacity: usize,
    data: VecDeque<T>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let data = VecDeque::with_capacity(capacity);
        Self {
            data,
            capacity,
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn full(&self) -> bool {
        self.size() >= self.capacity
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.full(){
            return  Err(Error::FullBuffer);
        }
        self.data.push_front(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.data.pop_back().unwrap().clone())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.full() {
            self.data.pop_back();
        }
        self.data.push_front(element);
    }
}
