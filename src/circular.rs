use std::iter;
/// CircularBuffer is used to store the last elements of an endless sequence.
/// Oldest elements will be overwritten. The implementation focus on
/// speed. So memory allocations are avoided.
///
/// Usage example:
///```
/// extern crate tui_logger;
///
/// use tui_logger::CircularBuffer;
///
/// let mut cb : CircularBuffer<u64> = CircularBuffer::new(5);
/// cb.push(1);
/// cb.push(2);
/// cb.push(3);
/// cb.push(4);
/// cb.push(5);
/// cb.push(6); // This will overwrite the first element
///
/// // Total elements pushed into the buffer is 6.
/// assert_eq!(6,cb.total_elements());
///
/// // Thus the buffer has wrapped around.
/// assert_eq!(true,cb.has_wrapped());
///
/// /// Iterate through the elements:
/// {
///     let mut iter = cb.iter();
///     assert_eq!(Some(&2), iter.next());
///     assert_eq!(Some(&3), iter.next());
///     assert_eq!(Some(&4), iter.next());
///     assert_eq!(Some(&5), iter.next());
///     assert_eq!(Some(&6), iter.next());
///     assert_eq!(None, iter.next());
/// }
///
/// /// Iterate backwards through the elements:
/// {
///     let mut iter = cb.rev_iter();
///     assert_eq!(Some(&6), iter.next());
///     assert_eq!(Some(&5), iter.next());
///     assert_eq!(Some(&4), iter.next());
///     assert_eq!(Some(&3), iter.next());
///     assert_eq!(Some(&2), iter.next());
///     assert_eq!(None, iter.next());
/// }
///
/// // The elements in the buffer are now:
/// assert_eq!(vec![2,3,4,5,6],cb.take());
///
/// // After taking all elements, the buffer is empty.
/// let now_empty : Vec<u64> = vec![];
/// assert_eq!(now_empty,cb.take());
///```
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    next_write_pos: usize,
}
#[allow(dead_code)]
impl<T> CircularBuffer<T> {
    /// Create a new CircularBuffer, which can hold max_depth elements
    pub fn new(max_depth: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: Vec::with_capacity(max_depth),
            next_write_pos: 0,
        }
    }
    /// Return the number of elements present in the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
    /// Next free index of the buffer
    pub fn first_index(&self) -> Option<usize> {
        if self.next_write_pos == 0 {
            None
        } else if self.next_write_pos < self.buffer.capacity() {
            Some(0)
        } else {
            Some(self.next_write_pos - self.buffer.capacity())
        }
    }
    pub fn last_index(&self) -> Option<usize> {
        if self.next_write_pos == 0 {
            None
        } else {
            Some(self.next_write_pos - 1)
        }
    }
    pub fn element_at_index(&self, index: usize) -> Option<&T> {
        let max_depth = self.buffer.capacity();
        if index >= self.next_write_pos {
            return None;
        }
        if index + max_depth < self.next_write_pos {
            return None;
        }
        Some(&self.buffer[index % max_depth])
    }
    /// Push a new element into the buffer.
    /// Until the capacity is reached, elements are pushed.
    /// Afterwards the oldest elements will be overwritten.
    pub fn push(&mut self, elem: T) {
        let max_depth = self.buffer.capacity();
        if self.buffer.len() < max_depth {
            self.buffer.push(elem);
        } else {
            self.buffer[self.next_write_pos % max_depth] = elem;
        }
        self.next_write_pos += 1;
    }
    /// Take out all elements from the buffer, leaving an empty buffer behind
    pub fn take(&mut self) -> Vec<T> {
        let mut consumed = vec![];
        let max_depth = self.buffer.capacity();
        if self.buffer.len() < max_depth {
            consumed.append(&mut self.buffer);
        } else {
            let pos = self.next_write_pos % max_depth;
            let mut xvec = self.buffer.split_off(pos);
            consumed.append(&mut xvec);
            consumed.append(&mut self.buffer)
        }
        self.next_write_pos = 0;
        consumed
    }
    /// Total number of elements pushed into the buffer.
    pub fn total_elements(&self) -> usize {
        self.next_write_pos
    }
    /// If has_wrapped() is true, then elements have been overwritten
    pub fn has_wrapped(&self) -> bool {
        self.next_write_pos > self.buffer.capacity()
    }
    /// Return an iterator to step through all elements in the sequence,
    /// as these have been pushed (FIFO)
    pub fn iter(&mut self) -> iter::Chain<std::slice::Iter<'_, T>, std::slice::Iter<'_, T>> {
        let max_depth = self.buffer.capacity();
        if self.next_write_pos <= max_depth {
            // If buffer is not completely filled, then just iterate through it
            self.buffer.iter().chain(self.buffer[..0].iter())
        } else {
            let wrap = self.next_write_pos % max_depth;
            let it_end = self.buffer[..wrap].iter();
            let it_start = self.buffer[wrap..].iter();
            it_start.chain(it_end)
        }
    }
    /// Return an iterator to step through all elements in the reverse sequence,
    /// as these have been pushed (LIFO)
    pub fn rev_iter(
        &mut self,
    ) -> iter::Chain<std::iter::Rev<std::slice::Iter<'_, T>>, std::iter::Rev<std::slice::Iter<'_, T>>>
    {
        let max_depth = self.buffer.capacity();
        if self.next_write_pos <= max_depth {
            // If buffer is not completely filled, then just iterate through it
            self.buffer
                .iter()
                .rev()
                .chain(self.buffer[..0].iter().rev())
        } else {
            let wrap = self.next_write_pos % max_depth;
            let it_end = self.buffer[..wrap].iter().rev();
            let it_start = self.buffer[wrap..].iter().rev();
            it_end.chain(it_start)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn circular_buffer() {
        use crate::CircularBuffer;

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);

        // Empty buffer
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), None);
            assert_eq!(cb.last_index(), None);
        }

        // Push in a value
        cb.push(1);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(0));
            assert_eq!(cb.last_index(), Some(0));
        }

        // Push in a value
        cb.push(2);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(0));
            assert_eq!(cb.last_index(), Some(1));
        }

        // Push in a value
        cb.push(3);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(0));
            assert_eq!(cb.last_index(), Some(2));
        }

        // Push in a value
        cb.push(4);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(0));
            assert_eq!(cb.last_index(), Some(3));
        }

        // Push in a value
        cb.push(5);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(0));
            assert_eq!(cb.last_index(), Some(4));
        }

        // Push in a value
        cb.push(6);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(1));
            assert_eq!(cb.last_index(), Some(5));
        }

        // Push in a value
        cb.push(7);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(2));
            assert_eq!(cb.last_index(), Some(6));
        }

        // Push in a value
        cb.push(8);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(3));
            assert_eq!(cb.last_index(), Some(7));
        }

        // Push in a value
        cb.push(9);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(4));
            assert_eq!(cb.last_index(), Some(8));
        }

        // Push in a value
        cb.push(10);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), Some(&10));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(5));
            assert_eq!(cb.last_index(), Some(9));
        }

        // Push in a value
        cb.push(11);
        {
            let mut cb_iter = cb.iter();
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), Some(&10));
            assert_eq!(cb_iter.next(), Some(&11));
            assert_eq!(cb_iter.next(), None);
            assert_eq!(cb.first_index(), Some(6));
            assert_eq!(cb.last_index(), Some(10));
            assert_eq!(cb.element_at_index(5), None);
            assert_eq!(cb.element_at_index(6), Some(&7));
            assert_eq!(cb.element_at_index(10), Some(&11));
            assert_eq!(cb.element_at_index(11), None);
        }
    }
    #[test]
    fn circular_buffer_rev() {
        use crate::CircularBuffer;

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);

        // Empty buffer
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(1);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(2);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(3);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(4);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(5);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), Some(&1));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(6);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), Some(&2));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(7);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), Some(&3));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(8);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), Some(&4));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(9);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), Some(&5));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(10);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&10));
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), Some(&6));
            assert_eq!(cb_iter.next(), None);
        }

        // Push in a value
        cb.push(11);
        {
            let mut cb_iter = cb.rev_iter();
            assert_eq!(cb_iter.next(), Some(&11));
            assert_eq!(cb_iter.next(), Some(&10));
            assert_eq!(cb_iter.next(), Some(&9));
            assert_eq!(cb_iter.next(), Some(&8));
            assert_eq!(cb_iter.next(), Some(&7));
            assert_eq!(cb_iter.next(), None);
        }
    }
    #[test]
    fn total_elements() {
        use crate::CircularBuffer;

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);

        assert_eq!(0, cb.total_elements());
        for i in 1..20 {
            cb.push(i);
            assert_eq!(i as usize, cb.total_elements());
        }
    }
    #[test]
    fn has_wrapped() {
        use crate::CircularBuffer;

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);

        assert_eq!(0, cb.total_elements());
        for i in 1..20 {
            cb.push(i);
            assert_eq!(i >= 6, cb.has_wrapped());
        }
    }
    #[test]
    fn take() {
        use crate::CircularBuffer;

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);
        for i in 1..5 {
            cb.push(i);
        }
        assert_eq!(vec![1, 2, 3, 4], cb.take());

        for i in 1..6 {
            cb.push(i);
        }
        assert_eq!(vec![1, 2, 3, 4, 5], cb.take());

        for i in 1..7 {
            cb.push(i);
        }
        assert_eq!(vec![2, 3, 4, 5, 6], cb.take());

        let mut cb: CircularBuffer<u64> = CircularBuffer::new(5);
        for i in 1..20 {
            cb.push(i);
        }
        assert_eq!(vec![15, 16, 17, 18, 19], cb.take());
    }
}
