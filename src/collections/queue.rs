use num_traits::Zero;

/// A fixed-size circular buffer/queue.
#[derive(Clone, Debug)]
pub struct Queue<T> {
    head: usize,
    buf: Vec<T>,
}

impl<T: Zero> Queue<T> {
    ///
    /// # Panics
    /// Panics if `cap == 0`
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "0-sized queue makes little sense?");
        Self {
            head: 0,
            buf: std::iter::repeat_with(|| T::zero())
                .take(capacity)
                .collect(),
        }
    }
}

impl<T> Queue<T> {
    pub fn as_slices(&self) -> (&[T], &[T]) {
        if self.head == 0 {
            (&self.buf, &[])
        } else {
            (&self.buf[self.head..], &self.buf[..self.head])
        }
    }

    pub fn push(&mut self, value: T) -> T {
        let res = std::mem::replace(&mut self.buf[self.head], value);
        self.head = (self.head + 1) % self.buf.len();
        res
    }
}

impl<T> Queue<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (a, b) = self.as_slices();
        ([a, b]).into_iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_queue_panics() {
        Queue::<i32>::with_capacity(0);
    }

    #[test]
    fn basic_usage() {
        let mut buf: Queue<i32> = Queue::with_capacity(5);
        assert_eq!(buf.as_slices(), (&[0, 0, 0, 0, 0][..], &[][..]));
        assert_eq!(buf.push(1), 0);
        assert_eq!(buf.as_slices(), (&[0, 0, 0, 0][..], &[1][..]));
        assert_eq!(buf.push(2), 0);
        assert_eq!(buf.as_slices(), (&[0, 0, 0][..], &[1, 2][..]));
        assert_eq!(buf.push(3), 0);
        assert_eq!(buf.as_slices(), (&[0, 0][..], &[1, 2, 3][..]));
        assert_eq!(buf.push(4), 0);
        assert_eq!(buf.as_slices(), (&[0][..], &[1, 2, 3, 4][..]));
        assert_eq!(buf.push(5), 0);
        assert_eq!(buf.as_slices(), (&[1, 2, 3, 4, 5][..], &[][..]));
        assert_eq!(buf.push(6), 1);
        assert_eq!(buf.as_slices(), (&[2, 3, 4, 5][..], &[6][..]));
        assert_eq!(buf.push(7), 2);
        assert_eq!(buf.as_slices(), (&[3, 4, 5][..], &[6, 7][..]));
    }

    #[test]
    fn iterate() {
        let mut buf: Queue<i32> = Queue::with_capacity(3);
        assert_eq!(buf.as_slices(), (&[0, 0, 0][..], &[][..]));
        assert_eq!(buf.iter().map(|&e| e).collect::<Vec<i32>>(), vec![0, 0, 0]);
        assert_eq!(buf.push(1), 0);
        assert_eq!(buf.iter().map(|&e| e).collect::<Vec<i32>>(), vec![0, 0, 1]);
        assert_eq!(buf.push(2), 0);
        assert_eq!(buf.iter().map(|&e| e).collect::<Vec<i32>>(), vec![0, 1, 2]);
        assert_eq!(buf.push(3), 0);
        assert_eq!(buf.iter().map(|&e| e).collect::<Vec<i32>>(), vec![1, 2, 3]);
        assert_eq!(buf.push(4), 1);
        assert_eq!(buf.iter().map(|&e| e).collect::<Vec<i32>>(), vec![2, 3, 4]);
    }
}
