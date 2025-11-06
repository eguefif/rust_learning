use crate::MyIterator;

pub struct Enumerate<I> {
    iter: I,
    index: usize,
}

impl<I> Enumerate<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter, index: 0
        }
    }
}

impl<I: MyIterator> MyIterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.iter.next() {
            let idx = self.index;
            self.index += 1;
            return Some((idx, value))
        }
        None
    }
}
