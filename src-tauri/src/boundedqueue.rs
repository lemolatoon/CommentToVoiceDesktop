use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct BoundedQueue<T, const N: usize> {
    inner: VecDeque<T>,
}

impl<T: Clone, const N: usize> BoundedQueue<T, N> {
    pub fn new(fill_value: T) -> Option<Self> {
        if N == 0 {
            return None;
        }
        let mut inner = VecDeque::with_capacity(usize::from(N));
        inner.resize_with(usize::from(N), || fill_value.clone());
        Some(Self { inner })
    }
}

impl<T, const N: usize> BoundedQueue<T, N> {
    pub fn push(&mut self, val: T) -> T {
        self.inner.push_back(val);
        let popped = self.inner.pop_front();
        debug_assert_eq!(self.inner.len(), usize::from(N));
        popped.unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (a, b) = self.inner.as_slices();
        a.iter().chain(b.iter())
    }
}

impl<T, const N: usize> IntoIterator for BoundedQueue<T, N> {
    type Item = T;

    type IntoIter = <VecDeque<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
