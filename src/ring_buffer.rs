pub struct RingBuffer<T> {
    buffer: Vec<T>,
    read_index: usize,
    write_index: usize,
}

impl<T: Copy + Default + PartialEq + std::fmt::Debug> RingBuffer<T> {
    pub fn new(length: usize) -> Self {
        let buffer = vec![T::default(); length];
        let read_index = 0;
        let write_index = 0;

        RingBuffer {
            buffer,
            read_index,
            write_index,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.iter_mut().for_each(|item| *item = T::default());
        self.read_index = 0;
        self.write_index = 0;
    } 

    pub fn put(&mut self, value: T) {
        self.buffer[self.write_index] = value;
    }

    pub fn peek(&self) -> T {
        self.buffer[self.read_index]
    }

    pub fn get(&self, offset: usize) -> T {
        self.buffer[(self.read_index + offset) % self.buffer.len()]
    }

    pub fn push(&mut self, value: T) {
        self.buffer[self.write_index] = value;
        self.write_index = (self.write_index + 1) % self.buffer.len();
    }

    pub fn pop(&mut self) -> T {
        let value = self.buffer[self.read_index];
        self.read_index = (self.read_index + 1) % self.buffer.len();
        value
    }

    pub fn get_read_index(&self) -> usize {
        self.read_index
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.read_index = index % self.buffer.len();
    }

    pub fn get_write_index(&self) -> usize {
        self.write_index
    }

    pub fn set_write_index(&mut self, index: usize) {
        self.write_index = index % self.buffer.len();
    }

    pub fn len(&self) -> usize {
        (self.write_index + self.buffer.len() - self.read_index) % self.buffer.len()
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        assert_eq!(buffer.pop(), 1);
        assert_eq!(buffer.pop(), 2);
        assert_eq!(buffer.pop(), 3);
    }

    #[test]
    fn test_peek() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);

        assert_eq!(buffer.peek(), 1);

        buffer.pop();

        assert_eq!(buffer.peek(), 2);
    }

    #[test]
    fn test_set_read_and_write_index() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);
        buffer.push(3);

        buffer.set_read_index(1);
        buffer.set_write_index(2);

        assert_eq!(buffer.get_read_index(), 1);
        assert_eq!(buffer.get_write_index(), 2);

        assert_eq!(buffer.get(0), 2);
        assert_eq!(buffer.get(1), 3);
    }

    #[test]
    fn test_reset() {
        let mut buffer = RingBuffer::new(3);

        buffer.push(1);
        buffer.push(2);

        buffer.reset();

        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.get_read_index(), 0);
        assert_eq!(buffer.get_write_index(), 0);
    }
}
