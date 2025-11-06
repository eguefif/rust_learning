use crate::MyIterator;

pub struct Take<I> {
    iter: I,
    n: usize,
    counter: usize,
}

impl<I> Take<I> {
    pub fn new(iter: I, n: usize) -> Self {
        Self {
            iter,
            n,
            counter: 0,
        }
    }
}

impl<I: MyIterator> MyIterator for Take<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.n {
            return None
        }
        self.counter += 1;
        return self.iter.next();
    }
}
